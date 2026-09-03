use uuid::Uuid;

use crate::{
    client::IikoClient,
    endpoints::internal_support::{DEFAULT_INTERNAL_RESPONSE_BYTES, read_internal},
    error::{IikoError, Result},
    xml::response::InternalReadResult,
};

/// The corporation scope served by the current iiko RMS node.
///
/// Document and stock APIs are authorized against `department_id`, so callers must not infer the
/// writable scope from the corporation hierarchy alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RmsCorporatedHierarchy {
    pub corporation_id: Uuid,
    pub legal_entity_id: Uuid,
    pub department_id: Uuid,
}

/// Allowlisted read-only corporation metadata required for safe routing decisions.
pub struct InternalCorporationEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InternalCorporationEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_rms_corporated_hierarchy(&self) -> Result<RmsCorporatedHierarchy> {
        let result = read_internal(
            self.client,
            "v3/CorporationService.getRMSCorporatedHierarchy",
            "<request/>",
            DEFAULT_INTERNAL_RESPONSE_BYTES,
        )
        .await?;
        parse_rms_corporated_hierarchy(result)
    }
}

fn parse_rms_corporated_hierarchy(result: InternalReadResult) -> Result<RmsCorporatedHierarchy> {
    let value = result.value.ok_or_else(|| {
        IikoError::XmlParsing("RMS corporation hierarchy response is empty".to_string())
    })?;
    Ok(RmsCorporatedHierarchy {
        corporation_id: required_uuid_child(&value, "corporation")?,
        legal_entity_id: required_uuid_child(&value, "jurPerson")?,
        department_id: required_uuid_child(&value, "department")?,
    })
}

fn required_uuid_child(
    value: &crate::xml::response::InternalXmlNode,
    child_name: &str,
) -> Result<Uuid> {
    let raw = value
        .child(child_name)
        .and_then(|child| child.text.as_deref())
        .ok_or_else(|| {
            IikoError::XmlParsing(format!("RMS corporation hierarchy is missing {child_name}"))
        })?;
    Uuid::parse_str(raw.trim()).map_err(|error| {
        IikoError::XmlParsing(format!(
            "RMS corporation hierarchy contains invalid {child_name}: {error}"
        ))
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::xml::response::InternalXmlNode;

    fn child(name: &str, value: &str) -> InternalXmlNode {
        InternalXmlNode {
            name: name.to_string(),
            attributes: BTreeMap::new(),
            text: Some(value.to_string()),
            children: Vec::new(),
        }
    }

    #[test]
    fn parses_verified_qitech_hierarchy_shape() {
        let corporation_id = Uuid::new_v4();
        let legal_entity_id = Uuid::new_v4();
        let department_id = Uuid::new_v4();
        let result = parse_rms_corporated_hierarchy(InternalReadResult {
            correlation_id: None,
            value: Some(InternalXmlNode {
                name: "r".to_string(),
                attributes: BTreeMap::new(),
                text: None,
                children: vec![
                    child("corporation", &corporation_id.to_string()),
                    child("jurPerson", &legal_entity_id.to_string()),
                    child("department", &department_id.to_string()),
                ],
            }),
        })
        .expect("hierarchy");

        assert_eq!(
            result,
            RmsCorporatedHierarchy {
                corporation_id,
                legal_entity_id,
                department_id,
            }
        );
    }

    #[test]
    fn rejects_incomplete_or_invalid_hierarchy() {
        let value = InternalXmlNode {
            name: "r".to_string(),
            attributes: BTreeMap::new(),
            text: None,
            children: vec![child("corporation", "not-a-uuid")],
        };
        assert!(
            parse_rms_corporated_hierarchy(InternalReadResult {
                correlation_id: None,
                value: Some(value),
            })
            .is_err()
        );
    }
}
