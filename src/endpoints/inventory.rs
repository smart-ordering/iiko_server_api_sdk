use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::request::InventoryRequest;
use crate::xml::response::InventoryItem;
use quick_xml::de::from_str;
use quick_xml::se::to_string;

pub struct InventoryEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> InventoryEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_inventory(&self, store_id: Option<String>) -> Result<Vec<InventoryItem>> {
        let request = InventoryRequest { store_id };
        let xml_body = to_string(&request)?;
        
        let response_xml = self.client.post_xml("inventory", &xml_body).await?;
        let items: Vec<InventoryItem> = from_str(&response_xml)?;
        
        Ok(items)
    }
}

