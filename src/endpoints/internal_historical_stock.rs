use chrono::NaiveDate;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{
        DEFAULT_INTERNAL_RESPONSE_BYTES, date_info_element, read_internal,
    },
    error::Result,
    xml::response::InternalReadResult,
};

/// Allowlisted historical stock snapshots from iiko's internal store service.
pub struct InternalHistoricalStockEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalHistoricalStockEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Returns iiko's product-balance map for one accounting date.
    pub async fn get_product_balances_for_date(
        &self,
        date: NaiveDate,
    ) -> Result<InternalReadResult> {
        read_internal(
            self.client,
            "v3/StoreService.getProductsBalanceForDate",
            &date_info_request(date),
            DEFAULT_INTERNAL_RESPONSE_BYTES,
        )
        .await
    }
}

fn date_info_request(date: NaiveDate) -> String {
    format!("<request>{}</request>", date_info_element("date", date))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_request_uses_verified_iiko_date_info_shape() {
        assert_eq!(
            date_info_request(NaiveDate::from_ymd_opt(2026, 7, 20).unwrap()),
            "<request><date><year>2026</year><month>7</month><day>20</day></date></request>"
        );
    }
}
