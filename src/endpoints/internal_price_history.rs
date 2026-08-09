use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{
        DEFAULT_INTERNAL_RESPONSE_BYTES, date_time_element, read_internal,
    },
    error::Result,
    xml::response::InternalReadResult,
};

/// Sale-price snapshots for one department and one point in time.
pub struct InternalPriceHistoryEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalPriceHistoryEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_department_snapshot(
        &self,
        department_id: Uuid,
        at: NaiveDateTime,
    ) -> Result<InternalReadResult> {
        let request = format!(
            "<request>{}<department>{department_id}</department></request>",
            date_time_element("date", at)
        );
        read_internal(
            self.client,
            "v3/ProductsService.getPriceListItemsByDepartment",
            &request,
            DEFAULT_INTERNAL_RESPONSE_BYTES,
        )
        .await
    }
}
