use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::request::{InventoryRequest, Request};
use crate::xml::response::InventoryItem;
use quick_xml::de::from_str;

pub struct InventoryEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InventoryEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_inventory(&self, store_id: Option<String>) -> Result<Vec<InventoryItem>> {
        let request = Request::new(InventoryRequest { store_id });
        let xml_body = request.to_xml()?;
        
        let response_xml = self.client.post_xml("inventory", &xml_body).await?;
        let items: Vec<InventoryItem> = from_str(&response_xml)?;
        
        Ok(items)
    }
}

