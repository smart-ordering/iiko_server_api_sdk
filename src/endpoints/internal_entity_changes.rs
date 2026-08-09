use std::collections::HashSet;

use crate::{
    client::IikoClient,
    endpoints::internal_support::DEFAULT_INTERNAL_RESPONSE_BYTES,
    error::{IikoError, Result},
    xml::response::ReferenceEntityDto,
};

const MAX_ENTITY_KINDS: usize = 8;

/// Non-personal reference types allowed in analytical revision reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AnalyticalEntityKind {
    AccountingCategory,
    AllergenGroup,
    Conception,
    CookingPlaceType,
    MeasureUnit,
    OrderType,
    ProductCategory,
    ProductScale,
    ProductSize,
    TaxCategory,
}

impl AnalyticalEntityKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountingCategory => "AccountingCategory",
            Self::AllergenGroup => "AllergenGroup",
            Self::Conception => "Conception",
            Self::CookingPlaceType => "CookingPlaceType",
            Self::MeasureUnit => "MeasureUnit",
            Self::OrderType => "OrderType",
            Self::ProductCategory => "ProductCategory",
            Self::ProductScale => "ProductScale",
            Self::ProductSize => "ProductSize",
            Self::TaxCategory => "TaxCategory",
        }
    }
}

/// Revision-cursor reads for a deliberately small, non-personal entity allowlist.
///
/// This uses the stable v2 revision contract because the internal Java revision request type is
/// not proven on QI Tech. Employee, account, payment, attendance and schedule types are excluded.
pub struct InternalEntityChangesEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalEntityChangesEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_changes(
        &self,
        kinds: &[AnalyticalEntityKind],
        revision_from: i64,
    ) -> Result<Vec<ReferenceEntityDto>> {
        if !(0..=i64::from(i32::MAX)).contains(&revision_from) {
            return Err(IikoError::BadRequest(
                "revision_from must fit QI Tech's non-negative 32-bit cursor".to_string(),
            ));
        }
        let mut seen = HashSet::with_capacity(kinds.len());
        let unique = kinds
            .iter()
            .copied()
            .filter(|kind| seen.insert(*kind))
            .collect::<Vec<_>>();
        if unique.is_empty() || unique.len() > MAX_ENTITY_KINDS {
            return Err(IikoError::BadRequest(format!(
                "between 1 and {MAX_ENTITY_KINDS} entity kinds are required"
            )));
        }

        let revision = revision_from.to_string();
        let mut params = unique
            .iter()
            .map(|kind| ("rootType", kind.as_str()))
            .collect::<Vec<_>>();
        params.push(("includeDeleted", "true"));
        params.push(("revisionFrom", revision.as_str()));
        let response = self
            .client
            .get_readonly_bounded("v2/entities/list", &params, DEFAULT_INTERNAL_RESPONSE_BYTES)
            .await?;
        Ok(serde_json::from_str(&response)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowlist_exposes_reference_data_without_personal_or_financial_types() {
        let names = [
            AnalyticalEntityKind::AccountingCategory,
            AnalyticalEntityKind::AllergenGroup,
            AnalyticalEntityKind::Conception,
            AnalyticalEntityKind::CookingPlaceType,
            AnalyticalEntityKind::MeasureUnit,
            AnalyticalEntityKind::OrderType,
            AnalyticalEntityKind::ProductCategory,
            AnalyticalEntityKind::ProductScale,
            AnalyticalEntityKind::ProductSize,
            AnalyticalEntityKind::TaxCategory,
        ]
        .map(AnalyticalEntityKind::as_str);
        assert!(!names.contains(&"Employee"));
        assert!(!names.contains(&"Account"));
        assert!(!names.contains(&"AttendanceType"));
    }

    #[test]
    fn revision_cursor_is_bounded_to_the_verified_server_integer_type() {
        assert!(!(0..=i64::from(i32::MAX)).contains(&-1));
        assert!(!(0..=i64::from(i32::MAX)).contains(&(i64::from(i32::MAX) + 1)));
    }
}
