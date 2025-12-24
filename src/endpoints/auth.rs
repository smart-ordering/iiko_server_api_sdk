use crate::client::IikoClient;
use crate::error::Result;

pub struct AuthEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> AuthEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn login(&self) -> Result<String> {
        self.client.authenticate().await
    }

    pub async fn logout(&self) -> Result<String> {
        self.client.logout().await
    }

    pub async fn invalidate_session(&self) {
        self.client.invalidate_session().await;
    }
}

