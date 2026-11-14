use super::*;
use crate::IikoConfig;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const GUEST_ID: Uuid = Uuid::from_u128(42);

async fn fixture(
    status: u16,
    body: String,
    advertised_length: Option<usize>,
) -> (IikoClient, tokio::task::JoinHandle<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let mut last_request = String::new();
        for is_auth in [true, false] {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut bytes = [0; 8192];
            let count = socket.read(&mut bytes).await.unwrap();
            last_request = String::from_utf8_lossy(&bytes[..count]).into_owned();
            let (status, payload, length) = if is_auth {
                (200, "fixture-session", "fixture-session".len())
            } else {
                (
                    status,
                    body.as_str(),
                    advertised_length.unwrap_or(body.len()),
                )
            };
            let headers = format!(
                "HTTP/1.1 {status} fixture\r\nContent-Length: {length}\r\nConnection: close\r\n\r\n"
            );
            let _ = socket.write_all(headers.as_bytes()).await;
            let _ = socket.write_all(payload.as_bytes()).await;
        }
        last_request
    });
    let client = IikoClient::new(IikoConfig::new(
        format!("http://{address}"),
        "fixture",
        "fixture",
    ))
    .unwrap();
    (client, server)
}

async fn search_guests(client: &IikoClient) -> Result<Vec<Employee>> {
    client
        .employees()
        .search(
            None,
            None,
            None,
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("Company"),
            None,
            None,
            None,
            Some(true),
        )
        .await
}

#[tokio::test]
async fn guest_search_preserves_role_filters_and_deleted_identity() {
    let xml = format!(
        "<employees><employee><id>{GUEST_ID}</id><name>Company</name><code>42</code><client>true</client><employee>false</employee><supplier>false</supplier><deleted>true</deleted></employee></employees>"
    );
    let (client, server) = fixture(200, xml, None).await;
    let guests = search_guests(&client).await.unwrap();
    assert_eq!(guests.len(), 1);
    assert_eq!(guests[0].id, GUEST_ID);
    assert!(guests[0].client);
    assert!(!guests[0].employee);
    assert_eq!(guests[0].deleted.as_deref(), Some("true"));
    let request = server.await.unwrap();
    assert!(request.starts_with("GET /employees/search?"));
    assert!(request.contains("client=true"));
    assert!(request.contains("includeDeleted=true"));
    assert!(request.contains("name=Company"));
}

#[tokio::test]
async fn exact_guest_identity_read_preserves_uuid_and_role() {
    let xml = format!(
        "<employee><id>{GUEST_ID}</id><name>Company</name><client>true</client></employee>"
    );
    let (client, server) = fixture(200, xml, None).await;
    let guest = client.employees().get_by_id(GUEST_ID).await.unwrap();
    assert_eq!(guest.id, GUEST_ID);
    assert!(guest.client);
    assert_eq!(guest.name, "Company");
    assert!(
        server
            .await
            .unwrap()
            .starts_with(&format!("GET /employees/byId/{GUEST_ID}?"))
    );
}

#[tokio::test]
async fn guest_search_and_identity_reads_bound_success_and_error_bodies() {
    for status in [200, 500] {
        for exact_identity in [false, true] {
            let (client, server) =
                fixture(status, String::new(), Some(EMPLOYEES_RESPONSE_BYTES + 1)).await;
            let error = if exact_identity {
                client.employees().get_by_id(GUEST_ID).await.unwrap_err()
            } else {
                search_guests(&client).await.unwrap_err()
            };
            assert!(error.to_string().contains("response exceeds"));
            server.await.unwrap();
        }
    }
}
