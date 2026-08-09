use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{
        DEFAULT_INTERNAL_RESPONSE_BYTES, date_info_element, read_internal,
    },
    error::Result,
    xml::response::InternalReadResult,
};

/// Read-only product usage data that can explain stock movement on one accounting date.
pub struct InternalStockMovementsEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalStockMovementsEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_product_usage(
        &self,
        date: NaiveDate,
        product_id: Uuid,
    ) -> Result<InternalReadResult> {
        let request = format!(
            "<request>{}<product>{product_id}</product></request>",
            date_info_element("date", date)
        );
        read_internal(
            self.client,
            "v3/StoreService.getProductUsagesByProduct",
            &request,
            DEFAULT_INTERNAL_RESPONSE_BYTES,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn usage_request_is_bounded_to_one_date_and_product() {
        let request = format!(
            "<request>{}<product>{}</product></request>",
            date_info_element("date", NaiveDate::from_ymd_opt(2026, 7, 20).unwrap()),
            uuid!("11111111-1111-1111-1111-111111111111")
        );
        assert_eq!(
            request,
            "<request><date><year>2026</year><month>7</month><day>20</day></date><product>11111111-1111-1111-1111-111111111111</product></request>"
        );
    }
}
