#![allow(dead_code)]

use iiko_server_api_sdk::{IikoClient, IikoConfig};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

pub fn assert_direct_loopback_environment() {
    for name in [
        "HTTP_PROXY",
        "HTTPS_PROXY",
        "ALL_PROXY",
        "http_proxy",
        "https_proxy",
        "all_proxy",
    ] {
        assert!(
            std::env::var_os(name).is_none_or(|value| value.is_empty()),
            "{name} must be unset for loopback transport contracts"
        );
    }
    // An explicit bypass also excludes OS-configured system proxies. Match the
    // same upper/lowercase precedence as reqwest rather than mutating global env.
    let bypass = std::env::var("NO_PROXY")
        .or_else(|_| std::env::var("no_proxy"))
        .unwrap_or_default();
    assert!(
        bypass
            .split(',')
            .any(|entry| matches!(entry.trim(), "*" | "127.0.0.1")),
        "set NO_PROXY=127.0.0.1,localhost for loopback transport contracts"
    );
}

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub target: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }

    pub fn query(&self) -> Vec<(String, String)> {
        reqwest::Url::parse(&format!("http://127.0.0.1{}", self.target))
            .unwrap()
            .query_pairs()
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect()
    }

    pub fn form(&self) -> Vec<(String, String)> {
        reqwest::Url::parse(&format!(
            "http://127.0.0.1/?{}",
            std::str::from_utf8(&self.body).unwrap()
        ))
        .unwrap()
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect()
    }
}

pub struct Reply {
    status: &'static str,
    headers: Vec<(&'static str, String)>,
    chunks: Vec<(Duration, Vec<u8>)>,
    header_delay: Duration,
    declared_length: Option<usize>,
    chunked: bool,
}

impl Reply {
    pub fn new(status: &'static str, body: impl AsRef<[u8]>) -> Self {
        Self {
            status,
            headers: Vec::new(),
            chunks: vec![(Duration::ZERO, body.as_ref().to_vec())],
            header_delay: Duration::ZERO,
            declared_length: None,
            chunked: false,
        }
    }

    pub fn ok(body: impl AsRef<[u8]>) -> Self {
        Self::new("200 OK", body)
    }

    pub fn header(mut self, name: &'static str, value: impl Into<String>) -> Self {
        self.headers.push((name, value.into()));
        self
    }

    pub fn delay_headers(mut self, delay: Duration) -> Self {
        self.header_delay = delay;
        self
    }

    pub fn body_chunks(mut self, chunks: Vec<(Duration, Vec<u8>)>) -> Self {
        self.chunks = chunks;
        self
    }

    pub fn chunked(mut self) -> Self {
        self.chunked = true;
        self
    }

    pub fn declared_length(mut self, length: usize) -> Self {
        self.declared_length = Some(length);
        self
    }

    async fn send(self, socket: &mut TcpStream) {
        tokio::time::sleep(self.header_delay).await;
        let mut header = format!("HTTP/1.1 {}\r\nConnection: close\r\n", self.status);
        if self.chunked {
            header.push_str("Transfer-Encoding: chunked\r\n");
        } else {
            let length = self
                .declared_length
                .unwrap_or_else(|| self.chunks.iter().map(|(_, body)| body.len()).sum());
            header.push_str(&format!("Content-Length: {length}\r\n"));
        }
        for (name, value) in self.headers {
            header.push_str(&format!("{name}: {value}\r\n"));
        }
        header.push_str("\r\n");
        if socket.write_all(header.as_bytes()).await.is_err() {
            return;
        }
        for (delay, body) in self.chunks {
            tokio::time::sleep(delay).await;
            if self.chunked
                && socket
                    .write_all(format!("{:x}\r\n", body.len()).as_bytes())
                    .await
                    .is_err()
            {
                return;
            }
            if socket.write_all(&body).await.is_err() {
                return;
            }
            if self.chunked && socket.write_all(b"\r\n").await.is_err() {
                return;
            }
        }
        if self.chunked {
            let _ = socket.write_all(b"0\r\n\r\n").await;
        }
    }
}

pub struct MockServer {
    pub base_url: String,
    task: Option<JoinHandle<Vec<Request>>>,
}

impl MockServer {
    pub async fn start(replies: Vec<Reply>) -> Self {
        assert_direct_loopback_environment();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let task = tokio::spawn(async move {
            let mut requests = Vec::new();
            for reply in replies {
                let (mut socket, _) =
                    tokio::time::timeout(Duration::from_secs(5), listener.accept())
                        .await
                        .expect("mock request was not sent")
                        .unwrap();
                requests.push(read_request(&mut socket).await);
                reply.send(&mut socket).await;
            }
            requests
        });
        Self {
            base_url: format!("http://{address}/resto/api"),
            task: Some(task),
        }
    }

    pub fn client(&self, timeout_secs: u64) -> IikoClient {
        IikoClient::new(IikoConfig::new(&self.base_url, "test", "test").with_timeout(timeout_secs))
            .unwrap()
    }

    pub async fn finish(mut self) -> Vec<Request> {
        tokio::time::timeout(Duration::from_secs(8), self.task.take().unwrap())
            .await
            .expect("mock server did not finish")
            .unwrap()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}

async fn read_request(socket: &mut TcpStream) -> Request {
    tokio::time::timeout(Duration::from_secs(3), async {
        let mut data = Vec::new();
        let mut buffer = [0; 4096];
        let boundary = loop {
            let size = socket.read(&mut buffer).await.unwrap();
            assert!(size > 0, "request ended before headers");
            data.extend_from_slice(&buffer[..size]);
            assert!(data.len() <= 128 * 1024, "unexpectedly large mock request");
            if let Some(index) = data.windows(4).position(|part| part == b"\r\n\r\n") {
                break index + 4;
            }
        };
        let header = std::str::from_utf8(&data[..boundary]).unwrap();
        let mut lines = header.split("\r\n");
        let mut first = lines.next().unwrap().split_whitespace();
        let method = first.next().unwrap().to_owned();
        let target = first.next().unwrap().to_owned();
        let headers: Vec<_> = lines
            .filter_map(|line| line.split_once(':'))
            .map(|(name, value)| (name.to_owned(), value.trim().to_owned()))
            .collect();
        let length = headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
            .map(|(_, value)| value.parse::<usize>().unwrap())
            .unwrap_or_default();
        assert!(boundary + length <= 128 * 1024);
        while data.len() < boundary + length {
            let size = socket.read(&mut buffer).await.unwrap();
            assert!(size > 0, "request ended before body");
            data.extend_from_slice(&buffer[..size]);
        }
        Request {
            method,
            target,
            headers,
            body: data[boundary..boundary + length].to_vec(),
        }
    })
    .await
    .expect("mock request read stalled")
}

pub fn pairs(items: &[(&str, &str)]) -> Vec<(String, String)> {
    items
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}
