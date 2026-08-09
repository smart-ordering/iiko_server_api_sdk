use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{
        DEFAULT_INTERNAL_RESPONSE_BYTES, date_time_element, read_internal, unique_bounded_ids,
        uuid_items,
    },
    error::Result,
    xml::response::InternalReadResult,
};

const MAX_STORE_IDS: usize = 20;

/// Historical cost snapshots for an explicitly bounded set of stores.
pub struct InternalCostHistoryEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalCostHistoryEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_last_costs_by_stores(
        &self,
        store_ids: &[Uuid],
        at: NaiveDateTime,
    ) -> Result<InternalReadResult> {
        let stores = unique_bounded_ids(store_ids, MAX_STORE_IDS, "store")?;
        let request = format!(
            "<request><stores>{}</stores>{}</request>",
            uuid_items(&stores),
            date_time_element("date", at)
        );
        read_internal(
            self.client,
            "v3/StoreService.getLastProductCostsByStores",
            &request,
            DEFAULT_INTERNAL_RESPONSE_BYTES,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use uuid::uuid;

    #[test]
    fn last_cost_request_has_bounded_store_collection_and_timestamp() {
        let stores = unique_bounded_ids(
            &[uuid!("11111111-1111-1111-1111-111111111111")],
            MAX_STORE_IDS,
            "store",
        )
        .unwrap();
        let at = NaiveDate::from_ymd_opt(2026, 7, 20)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(
            format!(
                "<request><stores>{}</stores>{}</request>",
                uuid_items(&stores),
                date_time_element("date", at)
            ),
            "<request><stores><i>11111111-1111-1111-1111-111111111111</i></stores><date>2026-07-20T00:00:00.000</date></request>"
        );
    }
}
