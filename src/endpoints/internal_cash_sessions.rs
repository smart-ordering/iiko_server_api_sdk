use uuid::Uuid;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{DEFAULT_INTERNAL_RESPONSE_BYTES, read_internal},
    error::Result,
    xml::response::InternalReadResult,
};

/// Read-only cash-session accounting data by one stable session identifier.
pub struct InternalCashSessionsEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalCashSessionsEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_transactions(&self, session_id: Uuid) -> Result<InternalReadResult> {
        read_internal(
            self.client,
            "v3/SessionsService.getSessionTransactions",
            &format!("<request><sessionId>{session_id}</sessionId></request>"),
            DEFAULT_INTERNAL_RESPONSE_BYTES,
        )
        .await
    }
}
