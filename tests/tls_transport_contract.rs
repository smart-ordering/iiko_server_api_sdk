mod http_contract_support;

use http_contract_support::{MockServer, Reply, assert_direct_loopback_environment, pairs};
use iiko_server_api_sdk::{IikoClient, IikoConfig, IikoError};
use std::fs;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};
use uuid::Uuid;

struct ProcessGuard(Child);

impl Drop for ProcessGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

struct TlsServer {
    directory: PathBuf,
    address: SocketAddr,
    process: Option<ProcessGuard>,
}

impl TlsServer {
    fn start() -> Self {
        assert_direct_loopback_environment();
        let directory = std::env::temp_dir().join(format!("iiko-sdk-tls-{}", Uuid::new_v4()));
        fs::create_dir(&directory).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&directory, fs::Permissions::from_mode(0o700)).unwrap();
        }
        // Install the directory guard before spawning anything or generating a key.
        let mut server = Self {
            directory,
            address: "127.0.0.1:0".parse().unwrap(),
            process: None,
        };
        server.run_openssl(
            "CA generation",
            &[
                "req",
                "-x509",
                "-newkey",
                "rsa:2048",
                "-sha256",
                "-nodes",
                "-days",
                "2",
                "-subj",
                "/CN=iiko-sdk-test-ca",
                "-addext",
                "basicConstraints=critical,CA:TRUE,pathlen:0",
                "-addext",
                "keyUsage=critical,keyCertSign,cRLSign",
                "-addext",
                "subjectKeyIdentifier=hash",
                "-keyout",
                "ca-key.pem",
                "-out",
                "ca.pem",
            ],
        );
        server.run_openssl(
            "leaf CSR generation",
            &[
                "req",
                "-new",
                "-newkey",
                "rsa:2048",
                "-sha256",
                "-nodes",
                "-subj",
                "/CN=localhost",
                "-keyout",
                "key.pem",
                "-out",
                "server.csr",
            ],
        );
        fs::write(
            server.directory.join("leaf.cnf"),
            concat!(
                "[server]\n",
                "basicConstraints=critical,CA:FALSE\n",
                "keyUsage=critical,digitalSignature,keyEncipherment\n",
                "extendedKeyUsage=serverAuth\n",
                "subjectAltName=DNS:localhost,IP:127.0.0.1\n",
                "subjectKeyIdentifier=hash\n",
                "authorityKeyIdentifier=keyid,issuer\n",
            ),
        )
        .unwrap();
        server.run_openssl(
            "leaf signing",
            &[
                "x509",
                "-req",
                "-in",
                "server.csr",
                "-CA",
                "ca.pem",
                "-CAkey",
                "ca-key.pem",
                "-CAcreateserial",
                "-out",
                "cert.pem",
                "-days",
                "1",
                "-sha256",
                "-extfile",
                "leaf.cnf",
                "-extensions",
                "server",
            ],
        );
        server.run_openssl(
            "server certificate verification",
            &[
                "verify",
                "-purpose",
                "sslserver",
                "-verify_ip",
                "127.0.0.1",
                "-CAfile",
                "ca.pem",
                "cert.pem",
            ],
        );
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        server.address = listener.local_addr().unwrap();
        drop(listener);
        fs::write(
            server.directory.join("response"),
            b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 7\r\nConnection: close\r\n\r\ntrusted",
        )
        .unwrap();
        server.process = Some(ProcessGuard(
            Command::new("openssl")
                .arg("s_server")
                .arg("-accept")
                .arg(server.address.to_string())
                .args([
                    "-cert", "cert.pem", "-key", "key.pem", "-HTTP", "-quiet", "-alpn", "http/1.1",
                ])
                .current_dir(&server.directory)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("openssl s_server could not start"),
        ));
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            assert!(
                server
                    .process
                    .as_mut()
                    .unwrap()
                    .0
                    .try_wait()
                    .unwrap()
                    .is_none(),
                "local TLS server exited before becoming ready"
            );
            if TcpStream::connect_timeout(&server.address, Duration::from_millis(100)).is_ok() {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "local TLS server readiness timed out"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
        server
    }

    fn url(&self, path: &str) -> String {
        format!("https://{}{path}", self.address)
    }

    fn root_certificate(&self) -> reqwest::Certificate {
        reqwest::Certificate::from_pem(&fs::read(self.directory.join("ca.pem")).unwrap()).unwrap()
    }

    fn run_openssl(&self, operation: &str, arguments: &[&str]) {
        let mut process = ProcessGuard(
            Command::new("openssl")
                .args(arguments)
                .current_dir(&self.directory)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("openssl is required for local TLS contract tests"),
        );
        let deadline = Instant::now() + Duration::from_secs(15);
        loop {
            if let Some(status) = process.0.try_wait().unwrap() {
                assert!(status.success(), "{operation} failed: {status}");
                break;
            }
            assert!(Instant::now() < deadline, "{operation} timed out");
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}

impl Drop for TlsServer {
    fn drop(&mut self) {
        // Stop and reap the process before removing the scoped private-key directory.
        drop(self.process.take());
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn assert_redacted_tls_failure(error: IikoError, secrets: &[&str]) {
    assert!(
        matches!(&error, IikoError::Http(_)),
        "wrong TLS error category: {error:?}"
    );
    let message = error.to_string();
    for secret in secrets {
        assert!(
            !message.contains(secret),
            "TLS transport error leaked request data"
        );
    }
    assert!(!message.contains("key="));
    assert!(!message.contains("https://"));
}

#[tokio::test]
async fn sdk_rejects_untrusted_certificate_and_redacts_https_auth_credentials() {
    let server = TlsServer::start();
    let login = "synthetic-tls-login";
    let hash = "synthetic-tls-password-hash";
    let client =
        IikoClient::new(IikoConfig::new(server.url("/resto/api"), login, hash).with_timeout(2))
            .unwrap();
    let error = tokio::time::timeout(Duration::from_secs(4), client.authenticate())
        .await
        .expect("TLS certificate rejection stalled")
        .unwrap_err();
    assert_redacted_tls_failure(error, &[login, hash, &server.address.to_string()]);

    // Prove that the synthetic server and certificate actually work. This factory
    // trusts a test CA explicitly; it does not prove SDK trust in an OS root store.
    let trusted = reqwest::Client::builder()
        .add_root_certificate(server.root_certificate())
        .http1_only()
        .no_proxy()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();
    let response = trusted.get(server.url("/response")).send().await.unwrap();
    assert_eq!(response.version(), reqwest::Version::HTTP_11);
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.text().await.unwrap(), "trusted");
}

#[tokio::test]
async fn authenticated_https_redirect_rejects_untrusted_cert_without_leaking_or_losing_key() {
    let tls = TlsServer::start();
    let secret = "synthetic-session-key-must-not-leak";
    let http = MockServer::start(vec![
        Reply::ok(secret),
        Reply::new("302 Found", "").header("Location", tls.url(&format!("/report?key={secret}"))),
        Reply::ok("later read"),
    ])
    .await;
    let client = http.client(2);
    let error = tokio::time::timeout(Duration::from_secs(4), client.get("report"))
        .await
        .expect("redirect TLS certificate rejection stalled")
        .unwrap_err();
    assert_redacted_tls_failure(error, &[secret, &tls.address.to_string()]);
    assert_eq!(client.get("after").await.unwrap(), "later read");
    let requests = http.finish().await;
    assert_eq!(requests.len(), 3);
    assert_eq!(requests[1].query(), pairs(&[("key", secret)]));
    assert_eq!(requests[2].query(), pairs(&[("key", secret)]));
}
