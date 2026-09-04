use super::*;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn delayed_server() -> (IikoClient, tokio::task::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let mut requests = Vec::new();
        for (delay, body) in [(0, "session"), (100, "<dayDishValues/>"), (100, "ordinary")] {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut data = [0; 4096];
            let n = socket.read(&mut data).await.unwrap();
            requests.push(String::from_utf8_lossy(&data[..n]).to_string());
            tokio::time::sleep(Duration::from_millis(delay)).await;
            let payload = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = socket.write_all(payload.as_bytes()).await;
        }
        requests
    });
    let mut client =
        IikoClient::new(IikoConfig::new(format!("http://{address}"), "test", "test")).unwrap();
    client.http_client = Client::builder()
        .timeout(Duration::from_millis(30))
        .build()
        .unwrap();
    (client, server)
}

#[tokio::test]
async fn expense_report_overrides_short_http_timeout_without_changing_other_requests() {
    let (client, server) = delayed_server().await;
    let expense = client
        .reports()
        .get_product_expense("scope", "11.07.2026", "11.07.2026", None, None)
        .await
        .unwrap();
    assert!(expense.is_empty());
    let ordinary = client.get("ordinary").await.unwrap_err();
    assert!(ordinary.to_string().contains("timed out"));
    let requests = server.await.unwrap();
    assert!(requests[0].starts_with("POST /auth"));
    assert!(requests[1].starts_with("GET /reports/productExpense?"));
    assert!(requests[1].contains("key=session"));
    assert!(requests[2].starts_with("GET /ordinary?key=session"));
}

#[tokio::test]
async fn bounded_override_does_not_bypass_response_size_guard() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut data = [0; 1024];
        socket.read(&mut data).await.unwrap();
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 100000000\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
    });
    let client =
        IikoClient::new(IikoConfig::new(format!("http://{address}"), "test", "test")).unwrap();
    *client.session_id.write().await = Some("preauthenticated".into());
    let error = client
        .reports()
        .get_product_expense("scope", "11.07.2026", "11.07.2026", None, None)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("exceeds"));
    server.await.unwrap();
}
