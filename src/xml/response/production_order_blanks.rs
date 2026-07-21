use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// iikoChain production-order blank returned by the internal v3 entity service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionOrderBlank {
    #[serde(rename = "@eid")]
    pub id: Uuid,
    pub revision: i64,
    pub deleted: bool,
    #[serde(rename = "blankName")]
    pub name: String,
    pub department: Uuid,
    #[serde(rename = "blankTabs")]
    pub tabs: ProductionOrderBlankTabs,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionOrderBlankTabs {
    #[serde(rename = "i", default)]
    pub items: Vec<ProductionOrderBlankTab>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionOrderBlankTab {
    pub name: String,
    pub store: Uuid,
    pub num: String,
    #[serde(rename = "blankItems")]
    pub items: ProductionOrderBlankItems,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionOrderBlankItems {
    #[serde(rename = "i", default)]
    pub items: Vec<ProductionOrderBlankItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionOrderBlankItem {
    pub id: Uuid,
    pub product: Uuid,
    pub position: String,
    #[serde(default)]
    pub comment: String,
    #[serde(rename = "containerId", default, deserialize_with = "empty_uuid")]
    pub container_id: Option<Uuid>,
    #[serde(rename = "excludedStores", default)]
    pub excluded_stores: ProductionOrderBlankExcludedStores,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionOrderBlankExcludedStores {
    #[serde(rename = "i", default)]
    pub items: Vec<Uuid>,
}

fn empty_uuid<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    value
        .filter(|value| !value.trim().is_empty())
        .map(|value| Uuid::parse_str(value.trim()).map_err(serde::de::Error::custom))
        .transpose()
}

#[derive(Debug, Deserialize)]
#[serde(rename = "result")]
pub(crate) struct ProductionOrderBlankServerResult {
    pub status: String,
    #[serde(rename = "resultValue", default)]
    pub result_value: Option<ProductionOrderBlankResultValue>,
    #[serde(rename = "errorsContainer", default)]
    pub errors: Option<ProductionOrderBlankErrorsContainer>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ProductionOrderBlankResultValue {
    pub r: ProductionOrderBlankResultCollection,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ProductionOrderBlankResultCollection {
    #[serde(rename = "i", default)]
    pub items: Vec<ProductionOrderBlank>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ProductionOrderBlankErrorsContainer {
    #[serde(rename = "rootError", default)]
    pub root_error: Option<String>,
}
