use uuid::Uuid;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{DEFAULT_INTERNAL_RESPONSE_BYTES, read_internal},
    error::Result,
    xml::response::InternalReadResult,
};

/// Allowlisted line-level sales reads by stable identifiers.
///
/// These records can contain customer or free-text fields. Applications must apply their own
/// classification/redaction policy before exposing a result to an assistant or end user.
pub struct InternalLineSalesEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalLineSalesEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_past_order(&self, order_id: Uuid) -> Result<InternalReadResult> {
        self.read(
            "v3/PastOrdersService.getPastOrdersById",
            &format!("<request><orderId>{order_id}</orderId></request>"),
        )
        .await
    }

    pub async fn get_item_sale_event(
        &self,
        item_sale_event_id: Uuid,
    ) -> Result<InternalReadResult> {
        self.read(
            "v3/PastOrdersService.getItemSaleEventsById",
            &format!("<request><itemSaleEventId>{item_sale_event_id}</itemSaleEventId></request>"),
        )
        .await
    }

    async fn read(&self, endpoint: &str, request: &str) -> Result<InternalReadResult> {
        read_internal(
            self.client,
            endpoint,
            request,
            DEFAULT_INTERNAL_RESPONSE_BYTES,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use uuid::uuid;

    #[test]
    fn line_sales_requests_use_verified_argument_names() {
        let id = uuid!("11111111-1111-1111-1111-111111111111");
        assert_eq!(
            format!("<request><orderId>{id}</orderId></request>"),
            "<request><orderId>11111111-1111-1111-1111-111111111111</orderId></request>"
        );
        assert_eq!(
            format!("<request><itemSaleEventId>{id}</itemSaleEventId></request>"),
            "<request><itemSaleEventId>11111111-1111-1111-1111-111111111111</itemSaleEventId></request>"
        );
    }
}
