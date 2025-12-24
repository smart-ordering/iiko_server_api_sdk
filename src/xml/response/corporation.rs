use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::common::IdName;

/// Типы подразделений
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DepartmentType {
    Corporation,
    Jurperson,
    Orgdevelopment,
    Department,
    Manufacture,
    Centralstore,
    Centraloffice,
    Salepoint,
    Store,
}

/// Элемент иерархии корпорации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateItemDto {
    #[serde(rename = "id")]
    pub id: Uuid,
    #[serde(rename = "parentId", default)]
    pub parent_id: Option<Uuid>,
    #[serde(rename = "code", default)]
    pub code: Option<String>,
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "taxpayerIdNumber", default)]
    pub taxpayer_id_number: Option<String>,
    #[serde(rename = "jurPersonAdditionalPropertiesDto", default)]
    pub jur_person_additional_properties: Option<JurPersonAdditionalPropertiesDto>,
}

/// Дополнительные свойства юридического лица
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurPersonAdditionalPropertiesDto {
    #[serde(rename = "taxpayerId", default)]
    pub taxpayer_id: Option<String>,
    #[serde(rename = "accountingReasonCode", default)]
    pub accounting_reason_code: Option<String>,
    #[serde(rename = "businessClassificationCode", default)]
    pub business_classification_code: Option<String>,
    #[serde(rename = "economicActivityClassificationCode", default)]
    pub economic_activity_classification_code: Option<String>,
    #[serde(rename = "iban", default)]
    pub iban: Option<String>,
    #[serde(rename = "swiftBic", default)]
    pub swift_bic: Option<String>,
    #[serde(rename = "registrationNumber", default)]
    pub registration_number: Option<String>,
    #[serde(rename = "address", default)]
    pub address: Option<String>,
    #[serde(rename = "settlementAccount", default)]
    pub settlement_account: Option<String>,
    #[serde(rename = "bik", default)]
    pub bik: Option<String>,
    #[serde(rename = "bank", default)]
    pub bank: Option<String>,
    #[serde(rename = "bankCity", default)]
    pub bank_city: Option<String>,
    #[serde(rename = "correspondentAccount", default)]
    pub correspondent_account: Option<String>,
    #[serde(rename = "phone", default)]
    pub phone: Option<String>,
    #[serde(rename = "gln", default)]
    pub gln: Option<String>,
    #[serde(rename = "description", default)]
    pub description: Option<String>,
    #[serde(rename = "legalAddressDto", default)]
    pub legal_address: Option<LegalAddressDto>,
    #[serde(rename = "officialEmployees", default)]
    pub official_employees: Option<OfficialEmployees>,
}

/// Юридический адрес
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalAddressDto {
    #[serde(rename = "zipCode", default)]
    pub zip_code: Option<String>,
    #[serde(rename = "office", default)]
    pub office: Option<String>,
    #[serde(rename = "building", default)]
    pub building: Option<String>,
    #[serde(rename = "house", default)]
    pub house: Option<String>,
    #[serde(rename = "street", default)]
    pub street: Option<String>,
    #[serde(rename = "community", default)]
    pub community: Option<String>,
    #[serde(rename = "city", default)]
    pub city: Option<String>,
    #[serde(rename = "district", default)]
    pub district: Option<String>,
    #[serde(rename = "region", default)]
    pub region: Option<String>,
    #[serde(rename = "country", default)]
    pub country: Option<String>,
}

/// Официальные сотрудники
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialEmployees {
    #[serde(rename = "i", default)]
    pub items: Vec<OfficialEmployeeDto>,
}

/// Официальный сотрудник
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialEmployeeDto {
    #[serde(rename = "role", default)]
    pub role: Option<String>,
    #[serde(rename = "name", default)]
    pub name: Option<String>,
}

/// Список элементов корпорации
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "corporateItemDtoes")]
pub struct CorporateItemDtoes {
    #[serde(rename = "corporateItemDto", default)]
    pub items: Vec<CorporateItemDto>,
}

/// Режим работы группы отделений
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GroupServiceMode {
    FastFood,
    TableService,
    Petroleum,
}

/// Группа отделений
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDto {
    #[serde(rename = "id")]
    pub id: Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "departmentId", default)]
    pub department_id: Option<Uuid>,
    #[serde(rename = "groupServiceMode")]
    pub group_service_mode: String,
    #[serde(rename = "pointOfSaleDtoes")]
    pub point_of_sale_dtoes: PointOfSaleDtoes,
    #[serde(rename = "restaurantSectionInfos", default)]
    pub restaurant_section_infos: Option<RestaurantSectionInfos>,
}

/// Список точек продаж
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointOfSaleDtoes {
    #[serde(rename = "pointOfSaleDto", default)]
    pub items: Vec<PointOfSaleDto>,
}

/// Точка продаж
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointOfSaleDto {
    #[serde(rename = "id")]
    pub id: Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "main")]
    pub main: bool,
    #[serde(rename = "cashRegisterInfo", default)]
    pub cash_register_info: Option<IdName>,
}

/// Информация о секциях ресторана
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantSectionInfos {
    #[serde(rename = "restaurantSectionInfo", default)]
    pub items: Vec<IdName>,
}

/// Список групп отделений
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "groupDtoes")]
pub struct GroupDtoes {
    #[serde(rename = "groupDto", default)]
    pub items: Vec<GroupDto>,
}

/// Терминал
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalDto {
    #[serde(rename = "id")]
    pub id: Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "computerName", default)]
    pub computer_name: Option<String>,
    #[serde(rename = "anonymous")]
    pub anonymous: bool,
    #[serde(rename = "groupInfo", default)]
    pub group_info: Option<IdName>,
    #[serde(rename = "restaurantSectionIds", default)]
    pub restaurant_section_ids: Option<RestaurantSectionIds>,
}

/// Список ID секций ресторана
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantSectionIds {
    #[serde(rename = "i", default)]
    pub items: Vec<String>,
}

/// Список терминалов
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "terminalDtoes")]
pub struct TerminalDtoes {
    #[serde(rename = "terminalDto", default)]
    pub items: Vec<TerminalDto>,
}

/// Настройки корпорации (JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporationSettings {
    #[serde(rename = "vatAccounting")]
    pub vat_accounting: String,
}

