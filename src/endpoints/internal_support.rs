use std::collections::HashSet;

use chrono::{Datelike, NaiveDate, NaiveDateTime};
use uuid::Uuid;

use crate::{
    client::IikoClient,
    error::{IikoError, Result},
    xml::response::{InternalReadResult, internal_read::parse_internal_server_result},
};

pub(crate) const DEFAULT_INTERNAL_RESPONSE_BYTES: usize = 4 * 1024 * 1024;

pub(crate) async fn read_internal(
    client: &IikoClient,
    endpoint: &str,
    request: &str,
    max_response_bytes: usize,
) -> Result<InternalReadResult> {
    let xml = client
        .post_xml_readonly_bounded(endpoint, request, max_response_bytes)
        .await?;
    parse_internal_server_result(&xml)
}

pub(crate) fn unique_bounded_ids(ids: &[Uuid], max_ids: usize, subject: &str) -> Result<Vec<Uuid>> {
    if ids.is_empty() {
        return Err(IikoError::BadRequest(format!(
            "at least one {subject} id is required"
        )));
    }
    let mut seen = HashSet::with_capacity(ids.len());
    let unique = ids
        .iter()
        .copied()
        .filter(|id| seen.insert(*id))
        .collect::<Vec<_>>();
    if unique.len() > max_ids {
        return Err(IikoError::BadRequest(format!(
            "at most {max_ids} {subject} ids are allowed"
        )));
    }
    Ok(unique)
}

pub(crate) fn uuid_items(ids: &[Uuid]) -> String {
    ids.iter()
        .map(|id| format!("<i>{id}</i>"))
        .collect::<String>()
}

pub(crate) fn date_info_element(name: &str, date: NaiveDate) -> String {
    format!(
        "<{name}><year>{}</year><month>{}</month><day>{}</day></{name}>",
        date.year(),
        date.month(),
        date.day()
    )
}

pub(crate) fn date_time_element(name: &str, date_time: NaiveDateTime) -> String {
    format!(
        "<{name}>{}</{name}>",
        date_time.format("%Y-%m-%dT%H:%M:%S%.3f")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn id_bounds_deduplicate_before_enforcing_limit() {
        let id = uuid!("11111111-1111-1111-1111-111111111111");
        assert_eq!(unique_bounded_ids(&[id, id], 1, "test").unwrap(), vec![id]);
        assert!(unique_bounded_ids(&[], 1, "test").is_err());
        assert!(unique_bounded_ids(&[id, Uuid::from_u128(2)], 1, "test").is_err());
    }

    #[test]
    fn date_helpers_match_verified_wire_shapes() {
        let date = NaiveDate::from_ymd_opt(2026, 7, 20).unwrap();
        assert_eq!(
            date_info_element("date", date),
            "<date><year>2026</year><month>7</month><day>20</day></date>"
        );
        assert_eq!(
            date_time_element("date", date.and_hms_opt(3, 4, 5).unwrap()),
            "<date>2026-07-20T03:04:05.000</date>"
        );
    }
}
