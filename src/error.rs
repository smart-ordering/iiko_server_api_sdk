use thiserror::Error;

#[derive(Error, Debug)]
pub enum IikoError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("XML deserialization error: {0}")]
    XmlDeserialization(#[from] quick_xml::DeError),

    #[error("XML serialization error: {0}")]
    XmlSerialization(#[from] quick_xml::SeError),

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

pub type Result<T> = std::result::Result<T, IikoError>;

