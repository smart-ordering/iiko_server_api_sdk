use thiserror::Error;

#[derive(Error, Debug)]
pub enum IikoError {
    #[error("HTTP request failed: {0}")]
    Http(String),

    #[error("XML deserialization error: {0}")]
    XmlDeserialization(#[from] quick_xml::DeError),

    #[error("XML serialization error: {0}")]
    XmlSerialization(#[from] quick_xml::SeError),

    #[error("XML parsing error: {0}")]
    XmlParsing(String),

    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Invalid configuration: {0}")]
    Configuration(String),

    #[error("API error (400 Bad Request): {0}")]
    BadRequest(String),

    #[error("API error (401 Unauthorized): {0}")]
    Unauthorized(String),

    #[error("API error (403 Forbidden): {0}")]
    Forbidden(String),

    #[error("API error (404 Not Found): {0}")]
    NotFound(String),

    #[error("API error (409 Conflict - Business Logic): {0}")]
    BusinessLogic(String),

    #[error("API error (500 Internal Server Error): {0}")]
    InternalServerError(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<reqwest::Error> for IikoError {
    fn from(error: reqwest::Error) -> Self {
        let category = if error.is_timeout() {
            "request timed out"
        } else if error.is_connect() {
            "connection failed"
        } else if error.is_decode() {
            "response decoding failed"
        } else if error.is_body() {
            "response body failed"
        } else if error.is_request() {
            "request construction failed"
        } else {
            "transport failed"
        };
        let status = error
            .status()
            .map(|status| format!(" (status {status})"))
            .unwrap_or_default();
        Self::Http(format!("{category}{status}"))
    }
}

pub type Result<T> = std::result::Result<T, IikoError>;

#[cfg(test)]
mod tests {
    use super::IikoError;
    use std::time::Duration;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn timeout_error_does_not_expose_url_or_session_key() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (_stream, _) = listener.accept().await.unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
        });
        let secret = "session-token-must-not-leak";
        let error = reqwest::Client::builder()
            .timeout(Duration::from_millis(25))
            .build()
            .unwrap()
            .get(format!("http://{address}/report?key={secret}"))
            .send()
            .await
            .unwrap_err();

        let message = IikoError::from(error).to_string();
        assert_eq!(message, "HTTP request failed: request timed out");
        assert!(!message.contains(secret));
        assert!(!message.contains("key="));
        assert!(!message.contains(&address.to_string()));
        server.abort();
    }
}
