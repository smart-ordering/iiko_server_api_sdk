use crate::config::IikoConfig;
use crate::error::{IikoError, Result};
use reqwest::{Client, Response, StatusCode};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// Клиент для работы с iiko Server API
///
/// # Важные ограничения:
/// - Запросы выполняются последовательно (не параллельно) согласно требованиям iiko API
/// - Каждый следующий запрос отправляется только после завершения предыдущего
/// - При авторизации занимается один слот лицензии - используйте `logout()` для освобождения
#[derive(Clone)]
pub struct IikoClient {
    config: Arc<IikoConfig>,
    http_client: Client,
    session_id: Arc<RwLock<Option<String>>>,
    /// Мьютекс для последовательного выполнения запросов
    /// Согласно документации iiko: "Запросы должны выполнятся последовательно друг за другом"
    request_mutex: Arc<Mutex<()>>,
}

impl IikoClient {
    pub fn new(config: IikoConfig) -> Result<Self> {
        // Если timeout_secs == 0 — не ставим таймаут на уровне HTTP-клиента (ожидаем, что таймауты обрабатывает вызывающий код)
        let http_client = if config.timeout_secs == 0 {
            Client::builder().build().map_err(|e| {
                IikoError::Configuration(format!("Failed to create HTTP client: {}", e))
            })?
        } else {
            let timeout = std::time::Duration::from_secs(config.timeout_secs);
            Client::builder().timeout(timeout).build().map_err(|e| {
                IikoError::Configuration(format!("Failed to create HTTP client: {}", e))
            })?
        };

        Ok(Self {
            config: Arc::new(config),
            http_client,
            session_id: Arc::new(RwLock::new(None)),
            request_mutex: Arc::new(Mutex::new(())),
        })
    }

    /// Внутренний метод аутентификации (без мьютекса)
    /// Используется внутри других методов, которые уже держат мьютекс
    async fn authenticate_internal(&self) -> Result<String> {
        let mut session = self.session_id.write().await;

        if let Some(ref sid) = *session {
            return Ok(sid.clone());
        }

        let url = format!("{}/auth", self.config.base_url);
        let form = [
            ("login", self.config.login.as_str()),
            ("pass", self.config.password.as_str()),
        ];

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(IikoError::Authentication(format!(
                "Authentication failed with status: {} - {}",
                status, error_text
            )));
        }

        let session_id = response.text().await?.trim().to_string();
        if session_id.is_empty() {
            return Err(IikoError::Authentication(
                "Empty token in response".to_string(),
            ));
        }

        *session = Some(session_id.clone());

        Ok(session_id)
    }

    /// Публичный метод аутентификации (защищен мьютексом)
    /// Используется для прямого вызова через auth().login()
    pub async fn authenticate(&self) -> Result<String> {
        // Последовательное выполнение запросов согласно требованиям iiko API
        let _guard = self.request_mutex.lock().await;
        self.authenticate_internal().await
    }

    pub async fn logout(&self) -> Result<String> {
        Ok(self.logout_if_authenticated().await?.unwrap_or_default())
    }

    /// Releases the current server-side session without creating a session solely to log it out.
    ///
    /// This is the safe primitive for session brokers and cache eviction. `None` means this
    /// client has not authenticated (or was already logged out), so no HTTP request was made.
    pub async fn logout_if_authenticated(&self) -> Result<Option<String>> {
        // Logout также должен быть последовательным.
        let _guard = self.request_mutex.lock().await;
        let Some(session_id) = self.session_id.read().await.clone() else {
            return Ok(None);
        };
        let url = format!("{}/logout", self.config.base_url);
        let form = [("key", session_id.as_str())];
        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form)
            .send()
            .await?;

        let status = response.status();
        if status == StatusCode::UNAUTHORIZED {
            // The server no longer recognizes the token, so it cannot own a live licensed
            // session. Clearing it locally is the safe idempotent logout outcome.
            self.invalidate_session().await;
            return Ok(None);
        }
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(IikoError::Api(format!(
                "Logout failed with status: {} - {}",
                status, error_text
            )));
        }

        let result = response.text().await?.trim().to_string();
        self.invalidate_session().await;
        Ok(Some(result))
    }

    fn handle_error_response(status: reqwest::StatusCode, error_text: String) -> IikoError {
        match status.as_u16() {
            400 => IikoError::BadRequest(error_text),
            401 => IikoError::Unauthorized(error_text),
            403 => IikoError::Forbidden(error_text),
            404 => IikoError::NotFound(error_text),
            409 => IikoError::BusinessLogic(error_text),
            500 => IikoError::InternalServerError(error_text),
            _ => IikoError::Api(format!(
                "Request failed with status: {} - {}",
                status, error_text
            )),
        }
    }

    async fn send_authenticated_get(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<Response> {
        let session_id = self.authenticate_internal().await?;
        let url = format!("{}/{}", self.config.base_url, endpoint);
        let mut query_params = vec![("key", session_id.as_str())];
        query_params.extend(params.iter().copied());

        Ok(self
            .http_client
            .get(&url)
            .query(&query_params)
            .send()
            .await?)
    }

    async fn authenticated_get_with_single_retry(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<String> {
        let mut response = self.send_authenticated_get(endpoint, params).await?;
        if response.status() == StatusCode::UNAUTHORIZED {
            self.invalidate_session().await;
            response = self.send_authenticated_get(endpoint, params).await?;
            if response.status() == StatusCode::UNAUTHORIZED {
                self.invalidate_session().await;
            }
        }

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Self::handle_error_response(status, error_text));
        }

        Ok(response.text().await?)
    }

    pub async fn get(&self, endpoint: &str) -> Result<String> {
        // Последовательное выполнение запросов согласно требованиям iiko API
        let _guard = self.request_mutex.lock().await;
        self.authenticated_get_with_single_retry(endpoint, &[])
            .await
    }

    pub async fn post_xml(&self, endpoint: &str, xml_body: &str) -> Result<String> {
        // Последовательное выполнение запросов согласно требованиям iiko API
        let _guard = self.request_mutex.lock().await;

        let session_id = self.authenticate_internal().await?;
        let url = format!("{}/{}", self.config.base_url, endpoint);

        let response = self
            .http_client
            .post(&url)
            .query(&[("key", session_id.as_str())])
            .header("Content-Type", "application/xml")
            .body(xml_body.to_string())
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Self::handle_error_response(status, error_text));
        }

        Ok(response.text().await?)
    }

    async fn send_authenticated_post_xml(
        &self,
        endpoint: &str,
        xml_body: &str,
    ) -> Result<Response> {
        let session_id = self.authenticate_internal().await?;
        let url = format!("{}/{}", self.config.base_url, endpoint);
        Ok(self
            .http_client
            .post(&url)
            .query(&[("key", session_id.as_str())])
            .header("Content-Type", "application/xml")
            .body(xml_body.to_string())
            .send()
            .await?)
    }

    /// Sends an explicitly read-only XML RPC and retries it once after an expired session.
    ///
    /// Callers must not use this for create/update/delete/process methods: replaying a mutating
    /// POST after an ambiguous response can duplicate the mutation. The ordinary `post_xml`
    /// intentionally remains non-retrying for that reason.
    pub(crate) async fn post_xml_readonly(&self, endpoint: &str, xml_body: &str) -> Result<String> {
        let _guard = self.request_mutex.lock().await;
        let mut response = self.send_authenticated_post_xml(endpoint, xml_body).await?;
        if response.status() == StatusCode::UNAUTHORIZED {
            self.invalidate_session().await;
            response = self.send_authenticated_post_xml(endpoint, xml_body).await?;
            if response.status() == StatusCode::UNAUTHORIZED {
                self.invalidate_session().await;
            }
        }

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Self::handle_error_response(status, error_text));
        }
        Ok(response.text().await?)
    }

    pub async fn put_xml(&self, endpoint: &str, xml_body: &str) -> Result<String> {
        // Последовательное выполнение запросов согласно требованиям iiko API
        let _guard = self.request_mutex.lock().await;

        let session_id = self.authenticate_internal().await?;
        let url = format!("{}/{}", self.config.base_url, endpoint);

        let response = self
            .http_client
            .put(&url)
            .query(&[("key", session_id.as_str())])
            .header("Content-Type", "application/xml")
            .body(xml_body.to_string())
            .send()
            .await?;

        let status = response.status();
        // PUT returns 200 for update, 201 for create
        if status != 200 && status != 201 {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Self::handle_error_response(status, error_text));
        }

        Ok(response.text().await?)
    }

    pub async fn post_form(&self, endpoint: &str, form_data: &[(&str, &str)]) -> Result<String> {
        // Последовательное выполнение запросов согласно требованиям iiko API
        let _guard = self.request_mutex.lock().await;

        let session_id = self.authenticate_internal().await?;
        let url = format!("{}/{}", self.config.base_url, endpoint);

        let query_params = vec![("key", session_id.as_str())];
        let form_params: Vec<(&str, &str)> = form_data.to_vec();

        let response = self
            .http_client
            .post(&url)
            .query(&query_params)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form_params)
            .send()
            .await?;

        let status = response.status();
        // POST returns 200 for update, 201 for create
        if status != 200 && status != 201 && !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Self::handle_error_response(status, error_text));
        }

        Ok(response.text().await?)
    }

    pub async fn post_json(
        &self,
        endpoint: &str,
        json_body: &str,
        query_params: &[(&str, &str)],
    ) -> Result<String> {
        // Последовательное выполнение запросов согласно требованиям iiko API
        let _guard = self.request_mutex.lock().await;

        let session_id = self.authenticate_internal().await?;
        let url = format!("{}/{}", self.config.base_url, endpoint);

        let mut all_params = vec![("key", session_id.as_str())];
        all_params.extend(query_params.iter().map(|(k, v)| (*k, *v)));

        let response = self
            .http_client
            .post(&url)
            .query(&all_params)
            .header("Content-Type", "application/json")
            .body(json_body.to_string())
            .send()
            .await?;

        let status = response.status();
        // POST returns 200 for update, 201 for create
        if status != 200 && status != 201 && !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Self::handle_error_response(status, error_text));
        }

        Ok(response.text().await?)
    }

    pub async fn get_with_params(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<String> {
        // Последовательное выполнение запросов согласно требованиям iiko API
        let _guard = self.request_mutex.lock().await;
        self.authenticated_get_with_single_retry(endpoint, params)
            .await
    }

    pub async fn delete(&self, endpoint: &str) -> Result<String> {
        // Последовательное выполнение запросов согласно требованиям iiko API
        let _guard = self.request_mutex.lock().await;

        let session_id = self.authenticate_internal().await?;
        let url = format!("{}/{}", self.config.base_url, endpoint);

        let response = self
            .http_client
            .delete(&url)
            .query(&[("key", session_id.as_str())])
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Self::handle_error_response(status, error_text));
        }

        Ok(response.text().await?)
    }

    pub fn config(&self) -> &IikoConfig {
        &self.config
    }

    pub async fn invalidate_session(&self) {
        let mut session = self.session_id.write().await;
        *session = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    struct MockResponse {
        status: &'static str,
        body: &'static str,
    }

    async fn spawn_mock_server(
        responses: Vec<MockResponse>,
    ) -> (String, tokio::task::JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            let mut requests = Vec::with_capacity(responses.len());
            for response in responses {
                let (mut stream, _) = listener.accept().await.unwrap();
                let mut request = Vec::new();
                let mut buffer = [0u8; 1024];
                while !request.windows(4).any(|window| window == b"\r\n\r\n") {
                    let read = stream.read(&mut buffer).await.unwrap();
                    if read == 0 {
                        break;
                    }
                    request.extend_from_slice(&buffer[..read]);
                }
                requests.push(String::from_utf8_lossy(&request).into_owned());

                let payload = format!(
                    "HTTP/1.1 {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    response.status,
                    response.body.len(),
                    response.body,
                );
                stream.write_all(payload.as_bytes()).await.unwrap();
            }
            requests
        });

        (format!("http://{address}"), handle)
    }

    fn client(base_url: String) -> IikoClient {
        IikoClient::new(IikoConfig::new(base_url, "user", "password")).unwrap()
    }

    #[tokio::test]
    async fn get_reauthenticates_once_after_cached_session_is_rejected() {
        let (base_url, server) = spawn_mock_server(vec![
            MockResponse {
                status: "200 OK",
                body: "expired-session",
            },
            MockResponse {
                status: "401 Unauthorized",
                body: "expired",
            },
            MockResponse {
                status: "200 OK",
                body: "fresh-session",
            },
            MockResponse {
                status: "200 OK",
                body: "payload",
            },
        ])
        .await;

        let result = client(base_url).get("resource").await.unwrap();
        let requests = server.await.unwrap();

        assert_eq!(result, "payload");
        assert!(requests[1].starts_with("GET /resource?key=expired-session "));
        assert!(requests[3].starts_with("GET /resource?key=fresh-session "));
    }

    #[tokio::test]
    async fn get_with_params_preserves_params_when_reauthenticating() {
        let (base_url, server) = spawn_mock_server(vec![
            MockResponse {
                status: "200 OK",
                body: "expired-session",
            },
            MockResponse {
                status: "401 Unauthorized",
                body: "expired",
            },
            MockResponse {
                status: "200 OK",
                body: "fresh-session",
            },
            MockResponse {
                status: "200 OK",
                body: "payload",
            },
        ])
        .await;

        let result = client(base_url)
            .get_with_params(
                "resource",
                &[("department", "main"), ("dateFrom", "01.01.2026")],
            )
            .await
            .unwrap();
        let requests = server.await.unwrap();

        assert_eq!(result, "payload");
        assert!(requests[1].contains("key=expired-session"));
        assert!(requests[1].contains("department=main"));
        assert!(requests[1].contains("dateFrom=01.01.2026"));
        assert!(requests[3].contains("key=fresh-session"));
        assert!(requests[3].contains("department=main"));
        assert!(requests[3].contains("dateFrom=01.01.2026"));
    }

    #[tokio::test]
    async fn get_returns_the_second_unauthorized_without_another_retry() {
        let (base_url, server) = spawn_mock_server(vec![
            MockResponse {
                status: "200 OK",
                body: "expired-session",
            },
            MockResponse {
                status: "401 Unauthorized",
                body: "expired",
            },
            MockResponse {
                status: "200 OK",
                body: "rejected-session",
            },
            MockResponse {
                status: "401 Unauthorized",
                body: "still unauthorized",
            },
        ])
        .await;

        let client = client(base_url);
        let error = client.get("resource").await.unwrap_err();
        let requests = server.await.unwrap();

        assert!(
            matches!(error, IikoError::Unauthorized(message) if message == "still unauthorized")
        );
        assert_eq!(requests.len(), 4);
        assert!(client.session_id.read().await.is_none());
    }

    #[tokio::test]
    async fn logout_without_a_session_does_not_authenticate() {
        let client = client("http://127.0.0.1:1".to_string());

        assert_eq!(client.logout_if_authenticated().await.unwrap(), None);
        assert_eq!(client.logout().await.unwrap(), "");
    }

    #[tokio::test]
    async fn logout_treats_an_expired_session_as_already_released() {
        let (base_url, server) = spawn_mock_server(vec![
            MockResponse {
                status: "200 OK",
                body: "expired-session",
            },
            MockResponse {
                status: "401 Unauthorized",
                body: "expired",
            },
        ])
        .await;
        let client = client(base_url);
        client.authenticate().await.unwrap();

        assert_eq!(client.logout_if_authenticated().await.unwrap(), None);
        assert!(client.session_id.read().await.is_none());
        assert_eq!(server.await.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn readonly_xml_post_reauthenticates_once() {
        let (base_url, server) = spawn_mock_server(vec![
            MockResponse {
                status: "200 OK",
                body: "expired-session",
            },
            MockResponse {
                status: "401 Unauthorized",
                body: "expired",
            },
            MockResponse {
                status: "200 OK",
                body: "fresh-session",
            },
            MockResponse {
                status: "200 OK",
                body: "<result><status>SUCCESS</status></result>",
            },
        ])
        .await;

        let result = client(base_url)
            .post_xml_readonly("v3/ReadService.getValue", "<request/>")
            .await
            .unwrap();
        let requests = server.await.unwrap();

        assert_eq!(result, "<result><status>SUCCESS</status></result>");
        assert!(requests[1].starts_with("POST /v3/ReadService.getValue?key=expired-session "));
        assert!(requests[3].starts_with("POST /v3/ReadService.getValue?key=fresh-session "));
        assert!(
            requests[1]
                .to_ascii_lowercase()
                .contains("content-type: application/xml")
        );
    }
}
