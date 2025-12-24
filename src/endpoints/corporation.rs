use crate::client::IikoClient;
use crate::error::Result;
use crate::xml::response::{
    CorporateItemDto, CorporateItemDtoes, CorporationSettings, GroupDto, GroupDtoes, TerminalDto,
    TerminalDtoes,
};
use quick_xml::de::from_str;

pub struct CorporationEndpoint<'a> {
    client: &'a IikoClient,
}

impl<'a> CorporationEndpoint<'a> {
    pub fn new(client: &'a IikoClient) -> Self {
        Self { client }
    }

    pub async fn get_departments(
        &self,
        revision_from: Option<i64>,
    ) -> Result<Vec<CorporateItemDto>> {
        let revision = revision_from.unwrap_or(-1).to_string();
        let response_xml = self
            .client
            .get_with_params("corporation/departments/", &[("revisionFrom", &revision)])
            .await?;

        let wrapper: CorporateItemDtoes = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    pub async fn get_stores(&self, revision_from: Option<i64>) -> Result<Vec<CorporateItemDto>> {
        let revision = revision_from.unwrap_or(-1).to_string();
        let response_xml = self
            .client
            .get_with_params("corporation/stores/", &[("revisionFrom", &revision)])
            .await?;

        let wrapper: CorporateItemDtoes = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    pub async fn get_groups(&self, revision_from: Option<i64>) -> Result<Vec<GroupDto>> {
        let revision = revision_from.unwrap_or(-1).to_string();
        let response_xml = self
            .client
            .get_with_params("corporation/groups/", &[("revisionFrom", &revision)])
            .await?;

        let wrapper: GroupDtoes = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    pub async fn get_terminals(&self, revision_from: Option<i64>) -> Result<Vec<TerminalDto>> {
        let revision = revision_from.unwrap_or(-1).to_string();
        let response_xml = self
            .client
            .get_with_params("corporation/terminals/", &[("revisionFrom", &revision)])
            .await?;

        let wrapper: TerminalDtoes = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    pub async fn search_department(&self, code: &str) -> Result<Option<CorporateItemDto>> {
        let response_xml = self
            .client
            .get_with_params("corporation/departments/search", &[("code", code)])
            .await?;

        let wrapper: CorporateItemDtoes = from_str(&response_xml)?;
        Ok(wrapper.items.into_iter().next())
    }

    pub async fn search_store(&self, code: &str) -> Result<Option<CorporateItemDto>> {
        let response_xml = self
            .client
            .get_with_params("corporation/stores/search", &[("code", code)])
            .await?;

        let wrapper: CorporateItemDtoes = from_str(&response_xml)?;
        Ok(wrapper.items.into_iter().next())
    }

    pub async fn search_groups(
        &self,
        name: Option<&str>,
        department_id: Option<&str>,
    ) -> Result<Vec<GroupDto>> {
        let mut params = Vec::new();
        if let Some(n) = name {
            params.push(("name", n));
        }
        if let Some(d) = department_id {
            params.push(("departmentId", d));
        }

        let response_xml = self
            .client
            .get_with_params("corporation/groups/search", &params)
            .await?;

        let wrapper: GroupDtoes = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    pub async fn search_terminals(
        &self,
        name: Option<&str>,
        computer_name: Option<&str>,
        anonymous: Option<bool>,
    ) -> Result<Vec<TerminalDto>> {
        let mut params = Vec::new();
        if let Some(n) = name {
            params.push(("name", n));
        }
        if let Some(c) = computer_name {
            params.push(("computerName", c));
        }
        if let Some(a) = anonymous {
            params.push(("anonymous", if a { "true" } else { "false" }));
        }

        let response_xml = self
            .client
            .get_with_params("corporation/terminals/search", &params)
            .await?;

        let wrapper: TerminalDtoes = from_str(&response_xml)?;
        Ok(wrapper.items)
    }

    pub async fn get_settings(&self) -> Result<CorporationSettings> {
        let response_json = self.client.get("v2/corporation/settings").await?;
        let settings: CorporationSettings = serde_json::from_str(&response_json)?;
        Ok(settings)
    }
}
