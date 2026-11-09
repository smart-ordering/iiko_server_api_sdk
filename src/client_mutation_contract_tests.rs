//! Synthetic HTTP contracts: no provider credentials, document writes or external network.
use super::*;
use std::{sync::Mutex as StdMutex, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::oneshot,
};

#[derive(Clone, Debug)]
struct Request {
    first_line: String,
    body: Vec<u8>,
}

struct Fixture {
    client: IikoClient,
    requests: Arc<StdMutex<Vec<Request>>>,
    stop: oneshot::Sender<()>,
    server: tokio::task::JoinHandle<()>,
}

impl Fixture {
    async fn new(response: Vec<u8>) -> Self {
        Self::with_journal(response, false).await
    }

    async fn with_journal(response: Vec<u8>, expire_first_read: bool) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(StdMutex::new(Vec::new()));
        let recorded = requests.clone();
        let (stop, mut stopped) = oneshot::channel();
        let server = tokio::spawn(async move {
            let mut auth_count = 0;
            let mut journal_reads = 0;
            loop {
                let (mut socket, _) = tokio::select! {
                    _ = &mut stopped => break,
                    accepted = listener.accept() => accepted.unwrap(),
                };
                let request =
                    tokio::time::timeout(Duration::from_secs(5), read_request(&mut socket))
                        .await
                        .unwrap();
                let auth = request.first_line.starts_with("POST /auth ");
                let journal = request.first_line.starts_with("POST /documents?");
                let redirected = request.first_line.starts_with("POST /redirected");
                recorded.lock().unwrap().push(request);
                let body = if auth {
                    auth_count += 1;
                    fixed(
                        "200 OK",
                        if auth_count == 1 {
                            b"synthetic-session"
                        } else {
                            b"refreshed-session"
                        },
                    )
                } else if journal && expire_first_read && journal_reads == 0 {
                    journal_reads += 1;
                    fixed("401 Unauthorized", b"expired synthetic session")
                } else if redirected {
                    fixed("200 OK", b"unexpected second destination")
                } else {
                    response.clone()
                };
                // An over-limit reader may close before the fixture finishes its body.
                let _ = socket.write_all(&body).await;
            }
        });
        let client = IikoClient::new(
            IikoConfig::new(format!("http://{address}"), "synthetic", "synthetic").with_timeout(2),
        )
        .unwrap();
        Self {
            client,
            requests,
            stop,
            server,
        }
    }

    async fn finish(self) -> Vec<Request> {
        let _ = self.stop.send(());
        self.server.await.unwrap();
        self.requests.lock().unwrap().clone()
    }
}

async fn read_request(socket: &mut TcpStream) -> Request {
    let mut headers = Vec::new();
    while !headers.ends_with(b"\r\n\r\n") {
        assert!(headers.len() < 16_384);
        headers.push(socket.read_u8().await.unwrap());
    }
    let text = String::from_utf8(headers).unwrap();
    let length = text
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().unwrap())
        })
        .unwrap_or_default();
    assert!(length <= 16_384);
    let mut body = vec![0; length];
    socket.read_exact(&mut body).await.unwrap();
    Request {
        first_line: text.lines().next().unwrap().to_owned(),
        body,
    }
}

fn fixed(status: &str, body: &[u8]) -> Vec<u8> {
    let mut response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    response.extend_from_slice(body);
    response
}

fn assert_one_dispatch(requests: &[Request]) {
    assert_eq!(requests.len(), 2, "{requests:?}");
    assert!(requests[0].first_line.starts_with("POST /auth "));
    assert!(
        requests[1]
            .first_line
            .starts_with("POST /documents/import/incomingInventory?key=synthetic-session ")
    );
    assert_eq!(requests[1].body, b"<document/>");
}

#[tokio::test]
async fn redirects_never_send_the_mutation_body_to_a_second_destination() {
    for status in ["307 Temporary Redirect", "308 Permanent Redirect"] {
        let response = format!("HTTP/1.1 {status}\r\nLocation: /redirected\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").into_bytes();
        let fixture = Fixture::new(response).await;
        assert!(
            fixture
                .client
                .post_xml_once_bounded("documents/import/incomingInventory", "<document/>", 64)
                .await
                .is_err()
        );
        assert_one_dispatch(&fixture.finish().await);
    }
}

#[tokio::test]
async fn unauthorized_mutation_neither_refreshes_authentication_nor_repeats_import() {
    let fixture = Fixture::new(fixed("401 Unauthorized", b"expired synthetic session")).await;
    let result = fixture
        .client
        .post_xml_once_bounded("documents/import/incomingInventory", "<document/>", 64)
        .await;
    assert!(matches!(result, Err(IikoError::Unauthorized(_))));
    assert_one_dispatch(&fixture.finish().await);
}

#[tokio::test]
async fn successful_and_error_bodies_are_bounded_before_decoding_or_error_mapping() {
    for status in ["200 OK", "500 Internal Server Error"] {
        for body in [
            format!("HTTP/1.1 {status}\r\nContent-Length: 100000000\r\nConnection: close\r\n\r\n").into_bytes(),
            format!("HTTP/1.1 {status}\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n8\r\n12345678\r\n9\r\n123456789\r\n0\r\n\r\n").into_bytes(),
        ] {
            let fixture = Fixture::new(body).await;
            let error = fixture.client.post_xml_once_bounded("documents/import/incomingInventory", "<document/>", 16).await.unwrap_err();
            assert!(matches!(error, IikoError::Api(message) if message.contains("exceeds 16 bytes")));
            assert_one_dispatch(&fixture.finish().await);
        }
    }
}

#[tokio::test]
async fn a_zero_response_limit_fails_before_authentication_or_dispatch() {
    let fixture = Fixture::new(fixed("200 OK", b"unreachable")).await;
    let result = fixture
        .client
        .post_xml_once_bounded("documents/import/incomingInventory", "<document/>", 0)
        .await;
    assert!(matches!(result, Err(IikoError::Configuration(_))));
    assert!(fixture.finish().await.is_empty());
}

#[tokio::test]
async fn document_journal_refreshes_an_expired_key_once_without_changing_the_read_body() {
    let document = b"<document><id>11111111-2222-3333-4444-555555555555</id><number>SYNTHETIC</number><date>2026-11-07</date><type>INCOMING_INVENTORY</type></document>";
    let fixture = Fixture::with_journal(fixed("200 OK", document), true).await;
    let documents = fixture
        .client
        .documents()
        .get_documents(
            Some("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee".into()),
            Some("2026-11-07".into()),
            Some("2026-11-07".into()),
        )
        .await
        .unwrap();
    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0].number, "SYNTHETIC");
    let requests = fixture.finish().await;
    assert_eq!(requests.len(), 4);
    assert!(requests[0].first_line.starts_with("POST /auth "));
    assert!(
        requests[1]
            .first_line
            .starts_with("POST /documents?key=synthetic-session ")
    );
    assert!(requests[2].first_line.starts_with("POST /auth "));
    assert!(
        requests[3]
            .first_line
            .starts_with("POST /documents?key=refreshed-session ")
    );
    assert_eq!(requests[1].body, requests[3].body);
    assert_eq!(requests[1].body, b"<request><storeId>aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee</storeId><dateFrom>2026-11-07</dateFrom><dateTo>2026-11-07</dateTo></request>");
}

#[tokio::test]
async fn document_journal_rejects_large_bodies_before_xml_deserialization() {
    let fixture = Fixture::new(
        b"HTTP/1.1 200 OK\r\nContent-Length: 100000000\r\nConnection: close\r\n\r\n".to_vec(),
    )
    .await;
    let error = fixture
        .client
        .documents()
        .get_documents(None, None, None)
        .await
        .unwrap_err();
    assert!(matches!(error, IikoError::Api(message) if message.contains("exceeds 4194304 bytes")));
    let requests = fixture.finish().await;
    assert_eq!(requests.len(), 2);
    assert!(requests[1].first_line.starts_with("POST /documents?"));
}
