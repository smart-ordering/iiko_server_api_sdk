use iiko_server_api_sdk::{IikoClient, IikoConfig};
use tokio::{io::{AsyncReadExt, AsyncWriteExt},net::TcpListener};
use std::time::Duration;

#[tokio::test]
async fn injected_transport_preserves_dns_override_and_rejects_redirects() {
    let first=TcpListener::bind("127.0.0.1:0").await.unwrap();
    let target=TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address=first.local_addr().unwrap();
    let target_address=target.local_addr().unwrap();
    let server=tokio::spawn(async move {
        let (mut stream,_)=first.accept().await.unwrap();
        let mut bytes=vec![0;8192];let n=stream.read(&mut bytes).await.unwrap();
        let request=String::from_utf8_lossy(&bytes[..n]);
        assert!(request.starts_with("POST /resto/api/auth "));
        let response=format!("HTTP/1.1 302 Found\r\nLocation: http://{target_address}/private\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
        stream.write_all(response.as_bytes()).await.unwrap();
    });
    let transport=reqwest::Client::builder().no_proxy().resolve("synthetic.invalid",address)
        .redirect(reqwest::redirect::Policy::none()).timeout(Duration::from_secs(2)).build().unwrap();
    let client=IikoClient::with_http_client(IikoConfig::new(format!("http://synthetic.invalid:{}/resto/api",address.port()),"synthetic","hash"),transport);
    assert!(client.auth().login().await.is_err());
    server.await.unwrap();
    assert!(tokio::time::timeout(Duration::from_millis(100),target.accept()).await.is_err());
}
