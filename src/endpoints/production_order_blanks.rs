use std::collections::HashSet;

use quick_xml::de::from_str;
use uuid::Uuid;

use crate::client::IikoClient;
use crate::error::{IikoError, Result};
use crate::xml::response::ProductionOrderBlank;
use crate::xml::response::production_order_blanks::ProductionOrderBlankServerResult;

const ENTITY_TYPE: &str = "ProductionOrderBlank";
const DETAIL_BATCH_SIZE: usize = 50;
const DETAIL_RESPONSE_BYTES: usize = 4 * 1024 * 1024;

pub struct ProductionOrderBlanksEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> ProductionOrderBlanksEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    /// Lists production-order blank identifiers through the public v2 endpoint.
    pub async fn get_ids(
        &self,
        include_deleted: bool,
        revision_from: Option<i64>,
    ) -> Result<Vec<Uuid>> {
        let include_deleted = if include_deleted { "true" } else { "false" };
        let revision = revision_from.map(|value| value.to_string());
        let mut params = vec![("includeDeleted", include_deleted)];
        if let Some(revision) = revision.as_deref() {
            params.push(("revisionFrom", revision));
        }
        let response = self
            .client
            .get_with_params(&format!("v2/entities/{ENTITY_TYPE}/ids"), &params)
            .await?;
        Ok(serde_json::from_str(&response)?)
    }

    /// Loads full production-order blanks through iikoChain's internal v3 entity service.
    ///
    /// The server fails a whole request when one UUID is invalid, so this method de-duplicates
    /// identifiers and sends bounded batches. It also validates the status inside ServerResult;
    /// an HTTP 200 alone does not mean that the entity call succeeded.
    pub async fn get_by_ids(&self, ids: &[Uuid]) -> Result<Vec<ProductionOrderBlank>> {
        let mut seen = HashSet::with_capacity(ids.len());
        let unique_ids = ids
            .iter()
            .copied()
            .filter(|id| seen.insert(*id))
            .collect::<Vec<_>>();
        let mut blanks = Vec::new();
        for batch in unique_ids.chunks(DETAIL_BATCH_SIZE) {
            let response = self
                .client
                .post_xml_readonly_bounded(
                    "v3/EntitiesService.getEntitiesByIds",
                    &build_entity_ids_request(batch),
                    DETAIL_RESPONSE_BYTES,
                )
                .await?;
            blanks.extend(parse_server_result(&response)?);
        }
        Ok(blanks)
    }

    /// Loads the current active production-order blank snapshot.
    pub async fn list_active(&self) -> Result<Vec<ProductionOrderBlank>> {
        let ids = self.get_ids(false, None).await?;
        self.get_by_ids(&ids).await
    }
}

fn build_entity_ids_request(ids: &[Uuid]) -> String {
    let ids = ids
        .iter()
        .map(|id| format!("<i>{id}</i>"))
        .collect::<String>();
    format!("<request><ids>{ids}</ids></request>")
}

fn parse_server_result(xml: &str) -> Result<Vec<ProductionOrderBlank>> {
    let result: ProductionOrderBlankServerResult = from_str(xml)?;
    if result.status != "SUCCESS" {
        let message = result
            .errors
            .and_then(|errors| errors.root_error)
            .filter(|message| !message.trim().is_empty())
            .unwrap_or_else(|| format!("iiko entity service returned {}", result.status));
        return Err(IikoError::Api(message));
    }
    Ok(result
        .result_value
        .map(|value| value.r.items)
        .unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::uuid;

    #[test]
    fn entity_ids_request_uses_i_collection_items() {
        assert_eq!(
            build_entity_ids_request(&[
                uuid!("11111111-1111-1111-1111-111111111111"),
                uuid!("22222222-2222-2222-2222-222222222222"),
            ]),
            "<request><ids><i>11111111-1111-1111-1111-111111111111</i><i>22222222-2222-2222-2222-222222222222</i></ids></request>"
        );
    }

    #[test]
    fn parses_production_order_blank_server_result() {
        let xml = r#"<result cls="ServerResult">
          <status>SUCCESS</status>
          <resultValue><r cls="java.util.ArrayList">
            <i cls="ProductionOrderBlank" eid="11111111-1111-1111-1111-111111111111">
              <revision>42</revision><deleted>false</deleted><blankName>Кухня</blankName>
              <department cls="Department">22222222-2222-2222-2222-222222222222</department>
              <blankTabs><i><name>Мясной цех</name><store>33333333-3333-3333-3333-333333333333</store><num>0</num>
                <blankItems><i><id>44444444-4444-4444-4444-444444444444</id><product>55555555-5555-5555-5555-555555555555</product><position>0</position><comment></comment><containerId null="1"></containerId><excludedStores><i>66666666-6666-6666-6666-666666666666</i></excludedStores></i></blankItems>
              </i></blankTabs>
            </i>
          </r></resultValue><errorsContainer><rootError null="1"></rootError></errorsContainer>
        </result>"#;

        let blanks = parse_server_result(xml).unwrap();
        assert_eq!(blanks.len(), 1);
        assert_eq!(blanks[0].name, "Кухня");
        assert_eq!(blanks[0].tabs.items[0].items.items[0].container_id, None);
        assert_eq!(
            blanks[0].tabs.items[0].items.items[0].excluded_stores.items,
            vec![uuid!("66666666-6666-6666-6666-666666666666")]
        );
    }

    #[test]
    fn rejects_server_result_failure_even_when_xml_is_valid() {
        let xml = r#"<result><status>SYSTEM_ERROR</status><errorsContainer><rootError>broken batch</rootError></errorsContainer></result>"#;
        let error = parse_server_result(xml).unwrap_err();
        assert!(error.to_string().contains("broken batch"));
    }
}
