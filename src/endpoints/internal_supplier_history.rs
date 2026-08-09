use chrono::NaiveDate;

use crate::{
    client::IikoClient,
    endpoints::{
        internal_support::DEFAULT_INTERNAL_RESPONSE_BYTES,
        suppliers::parse_supplier_pricelist_response,
    },
    error::{IikoError, Result},
    xml::response::SupplierPriceListItemDto,
};

/// Historical supplier pricelists through iiko's documented, narrower fallback route.
///
/// The similarly named internal v3 service is intentionally not used: its supplier collection
/// wire type is not stable across the tested server and plausible UUID shapes fail at runtime.
pub struct InternalSupplierHistoryEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalSupplierHistoryEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_pricelist_on_date(
        &self,
        supplier_code: &str,
        date: NaiveDate,
    ) -> Result<Vec<SupplierPriceListItemDto>> {
        validate_supplier_code(supplier_code)?;
        let date = date.format("%d.%m.%Y").to_string();
        let endpoint = format!("suppliers/{supplier_code}/pricelist");
        let response = self
            .client
            .get_readonly_bounded(
                &endpoint,
                &[("date", date.as_str())],
                DEFAULT_INTERNAL_RESPONSE_BYTES,
            )
            .await?;
        parse_supplier_pricelist_response(&response)
    }
}

fn validate_supplier_code(code: &str) -> Result<()> {
    if code.is_empty()
        || code.len() > 64
        || !code
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(IikoError::BadRequest(
            "supplier code must contain 1-64 ASCII letters, digits, '.', '-' or '_'".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supplier_code_cannot_escape_the_endpoint_path() {
        assert!(validate_supplier_code("supplier-01").is_ok());
        assert!(validate_supplier_code("../employees").is_err());
        assert!(validate_supplier_code("code/other").is_err());
        assert!(validate_supplier_code("").is_err());
    }
}
