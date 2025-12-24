use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::response::{ReplicationStatus, ServerType};
use quick_xml::de::from_str;
use serde::Deserialize;
use uuid::Uuid;

pub struct ReplicationEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> ReplicationEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_statuses(&self) -> Result<Vec<ReplicationStatus>> {
        let response_xml = self
            .client
            .get("replication/statuses")
            .await?;

        let wrapper: crate::xml::response::ReplicationStatuses = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    pub async fn get_status_by_department(&self, department_id: Uuid) -> Result<ReplicationStatus> {
        let endpoint = format!("replication/byDepartmentId/{}/status", department_id);
        let response_xml = self.client.get(&endpoint).await?;

        let status: ReplicationStatus = from_str(&response_xml)?;
        Ok(status)
    }

    pub async fn get_server_type(&self) -> Result<ServerType> {
        let response_xml = self.client.get("replication/serverType").await?;
        
        // Response is XML: <serverType>CHAIN</serverType>
        #[derive(Debug, Deserialize)]
        #[serde(rename = "serverType")]
        struct ServerTypeWrapper {
            #[serde(rename = "$text")]
            value: String,
        }
        
        let wrapper: ServerTypeWrapper = from_str(&response_xml)?;
        let server_type_str = wrapper.value.trim();
        
        match server_type_str {
            "CHAIN" => Ok(ServerType::Chain),
            "REPLICATED_RMS" => Ok(ServerType::ReplicatedRms),
            "STANDALONE_RMS" => Ok(ServerType::StandaloneRms),
            _ => Err(crate::error::IikoError::Api(format!(
                "Unknown server type: {}",
                server_type_str
            ))),
        }
    }
}

