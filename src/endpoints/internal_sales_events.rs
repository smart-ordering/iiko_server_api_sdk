use uuid::Uuid;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{DEFAULT_INTERNAL_RESPONSE_BYTES, read_internal},
    error::Result,
    xml::response::InternalReadResult,
};

/// Stable-ID sales event reads. Results may contain personal or free-text fields.
pub struct InternalSalesEventsEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalSalesEventsEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_item_sale_event(
        &self,
        item_sale_event_id: Uuid,
    ) -> Result<InternalReadResult> {
        read_internal(
            self.client,
            "v3/PastOrdersService.getItemSaleEventsById",
            &format!("<request><itemSaleEventId>{item_sale_event_id}</itemSaleEventId></request>"),
            DEFAULT_INTERNAL_RESPONSE_BYTES,
        )
        .await
    }
}
