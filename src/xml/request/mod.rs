use quick_xml::se::to_string;
use serde::Serialize;

pub mod events;

#[derive(Debug, Serialize)]
#[serde(rename = "request")]
pub struct Request<T> {
    #[serde(flatten)]
    pub body: T,
}

impl<T: Serialize> Request<T> {
    pub fn new(body: T) -> Self {
        Self { body }
    }

    pub fn to_xml(&self) -> Result<String, quick_xml::SeError> {
        to_string(self)
    }
}

#[derive(Debug, Serialize)]
pub struct InventoryRequest {
    #[serde(rename = "storeId")]
    pub store_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SuppliersRequest {
    #[serde(rename = "storeId")]
    pub store_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DocumentsRequest {
    #[serde(rename = "storeId")]
    pub store_id: Option<String>,
    #[serde(rename = "dateFrom")]
    pub date_from: Option<String>,
    #[serde(rename = "dateTo")]
    pub date_to: Option<String>,
}

