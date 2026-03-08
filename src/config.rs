use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IikoConfig {
    pub base_url: String,
    pub login: String,
    pub password: String,
    pub timeout_secs: u64,
}

impl Default for IikoConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8080/resto/api".to_string(),
            login: String::new(),
            password: String::new(),
            // 0 означает отсутствие таймаута на уровне HTTP-клиента
            timeout_secs: 0,
        }
    }
}

impl IikoConfig {
    pub fn new(
        base_url: impl Into<String>,
        login: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            login: login.into(),
            password: password.into(),
            // 0 означает отсутствие таймаута на уровне HTTP-клиента
            timeout_secs: 0,
        }
    }

    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let base_url = std::env::var("IIKO_BASE_URL").expect("IIKO_BASE_URL is not set");
        let login = std::env::var("IIKO_LOGIN").expect("IIKO_LOGIN is not set");
        let password =
            std::env::var("IIKO_HASHED_PASSWORD").expect("IIKO_HASHED_PASSWORD is not set");

        Ok(Self::new(base_url, login, password))
    }
}
