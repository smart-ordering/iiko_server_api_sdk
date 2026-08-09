use uuid::Uuid;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{DEFAULT_INTERNAL_RESPONSE_BYTES, unique_bounded_ids},
    error::{IikoError, Result},
    xml::response::ProductionOrderBlank,
};

const MAX_PRODUCTION_ORDER_IDS: usize = 50;

/// Read-only production-order definitions that anchor later document tracing.
///
/// Linked production documents remain unavailable until their internal collection type is
/// positively verified. This endpoint does not claim that definitions are execution events.
pub struct InternalProductionTraceEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalProductionTraceEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_order_definition_ids(&self, revision_from: Option<i64>) -> Result<Vec<Uuid>> {
        Self::validate_revision_cursor(revision_from)?;
        let revision = revision_from.map(|value| value.to_string());
        let mut params = vec![("includeDeleted", "false")];
        if let Some(revision) = revision.as_deref() {
            params.push(("revisionFrom", revision));
        }
        let response = self
            .client
            .get_readonly_bounded(
                "v2/entities/ProductionOrderBlank/ids",
                &params,
                DEFAULT_INTERNAL_RESPONSE_BYTES,
            )
            .await?;
        Ok(serde_json::from_str(&response)?)
    }

    pub async fn get_order_definitions(
        &self,
        production_order_ids: &[Uuid],
    ) -> Result<Vec<ProductionOrderBlank>> {
        let ids = unique_bounded_ids(
            production_order_ids,
            MAX_PRODUCTION_ORDER_IDS,
            "production order",
        )?;
        self.client.production_order_blanks().get_by_ids(&ids).await
    }

    fn validate_revision_cursor(revision_from: Option<i64>) -> Result<()> {
        if revision_from.is_some_and(|value| !(0..=i64::from(i32::MAX)).contains(&value)) {
            return Err(IikoError::BadRequest(
                "revision_from must fit QI Tech's non-negative 32-bit cursor".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revision_cursor_matches_verified_server_integer_type() {
        assert!(InternalProductionTraceEndpoint::validate_revision_cursor(Some(-1)).is_err());
        assert!(
            InternalProductionTraceEndpoint::validate_revision_cursor(Some(
                i64::from(i32::MAX) + 1
            ))
            .is_err()
        );
        assert!(InternalProductionTraceEndpoint::validate_revision_cursor(None).is_ok());
        assert!(InternalProductionTraceEndpoint::validate_revision_cursor(Some(42)).is_ok());
    }
}
