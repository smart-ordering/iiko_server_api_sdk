use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

/// Сериализация bool в строку "true" или "false" для XML
fn serialize_bool_as_string<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(if *value { "true" } else { "false" })
}

/// Десериализация строки "true"/"false" в bool
fn deserialize_bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" | "" => Ok(false),
        _ => Ok(false), // По умолчанию false
    }
}

/// Сотрудник (Employee)
///
/// # Согласно документации iiko API v1.0+:
/// Структура соответствует XSD Сотрудник (employee)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "employee", default)]
pub struct Employee {
    /// UUID сотрудника
    #[serde(rename = "id")]
    pub id: Uuid,
    /// Табельный номер сотрудника. Пуст у системных учетных записей
    #[serde(rename = "code")]
    pub code: String,
    /// Имя сотрудника
    #[serde(rename = "name")]
    pub name: String,
    /// Логин для входа в бекофис
    #[serde(rename = "login", default)]
    pub login: Option<String>,
    /// Пароль для входа в бекофис. Доступен только на изменение, не отображается
    #[serde(rename = "password", default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Основная должность сотрудника (UUID). Входит в rolesIds
    #[serde(rename = "mainRoleId", default)]
    pub main_role_id: Option<Uuid>,
    /// UUID ролей
    #[serde(rename = "rolesIds", default)]
    pub roles_ids: Option<Vec<Uuid>>,
    /// Основная должность сотрудника (код). Входит в roleCodes (кроме системных учетных записей)
    #[serde(rename = "mainRoleCode", default)]
    pub main_role_code: Option<String>,
    /// Коды ролей
    #[serde(rename = "roleCodes", default)]
    pub role_codes: Option<Vec<String>>,
    /// Телефон
    #[serde(rename = "phone", default)]
    pub phone: Option<String>,
    /// Мобильный телефон
    #[serde(rename = "cellPhone", default)]
    pub cell_phone: Option<String>,
    /// Имя
    #[serde(rename = "firstName", default)]
    pub first_name: Option<String>,
    /// Отчество
    #[serde(rename = "middleName", default)]
    pub middle_name: Option<String>,
    /// Фамилия
    #[serde(rename = "lastName", default)]
    pub last_name: Option<String>,
    /// Дата рождения
    #[serde(rename = "birthday", default)]
    pub birthday: Option<String>,
    /// Email
    #[serde(rename = "email", default)]
    pub email: Option<String>,
    /// Адрес
    #[serde(rename = "address", default)]
    pub address: Option<String>,
    /// Дата приема на работу
    #[serde(rename = "hireDate", default)]
    pub hire_date: Option<String>,
    /// Номер документа приема
    #[serde(rename = "hireDocumentNumber", default)]
    pub hire_document_number: Option<String>,
    /// Дата увольнения (с версии 5.4)
    #[serde(rename = "fireDate", default)]
    pub fire_date: Option<String>,
    /// Примечание
    #[serde(rename = "note", default)]
    pub note: Option<String>,
    /// Номер карты сотрудника (Slip карты)
    #[serde(rename = "cardNumber", default)]
    pub card_number: Option<String>,
    /// Pin-код для входа в iikoFront. Доступен только на изменение, не отображается
    #[serde(rename = "pinCode", default, skip_serializing_if = "Option::is_none")]
    pub pin_code: Option<String>,
    /// ИНН
    #[serde(rename = "taxpayerIdNumber", default)]
    pub taxpayer_id_number: Option<String>,
    /// СНИЛС (с версии 5.4)
    #[serde(rename = "snils", default)]
    pub snils: Option<String>,
    /// Global Location Number для поставщиков (с версии 6.0)
    #[serde(rename = "gln", default, skip_serializing_if = "Option::is_none")]
    pub gln: Option<String>,
    /// Дата активации
    #[serde(rename = "activationDate", default)]
    pub activation_date: Option<String>,
    /// Дата деактивации
    #[serde(rename = "deactivationDate", default)]
    pub deactivation_date: Option<String>,
    /// Предпочтительное подразделение (одно из departmentCodes), в котором сотруднику назначаются смены в первую очередь (с версии 5.0)
    #[serde(rename = "preferredDepartmentCode", default)]
    pub preferred_department_code: Option<String>,
    /// Назначенные подразделения. Если null - сотруднику назначены все подразделения (существующие и будущие)
    #[serde(rename = "departmentCodes", default)]
    pub department_codes: Option<Vec<String>>,
    /// Подразделения, в которых сотрудник является ответственным. Если null - сотрудник является ответственным во всех существующих и будущих подразделениях
    #[serde(rename = "responsibilityDepartmentCodes", default)]
    pub responsibility_department_codes: Option<Vec<String>>,
    /// Удален
    #[serde(rename = "deleted", default)]
    pub deleted: Option<String>,
    /// Признак поставщика
    #[serde(
        rename = "supplier",
        default,
        serialize_with = "serialize_bool_as_string",
        deserialize_with = "deserialize_bool_from_string"
    )]
    pub supplier: bool,
    /// Признак сотрудника
    #[serde(
        rename = "employee",
        default,
        serialize_with = "serialize_bool_as_string",
        deserialize_with = "deserialize_bool_from_string"
    )]
    pub employee: bool,
    /// Признак клиента
    #[serde(
        rename = "client",
        default,
        serialize_with = "serialize_bool_as_string",
        deserialize_with = "deserialize_bool_from_string"
    )]
    pub client: bool,
    /// Произвольные данные в формате строковый ключ -> строковое значение,
    /// семантика которых определяется внешней системой.
    /// Сервер никак не интерпретирует эти данные.
    #[serde(rename = "publicExternalData", default)]
    pub public_external_data: Option<PublicExternalData>,
}

impl Default for Employee {
    fn default() -> Self {
        Self {
            id: Uuid::nil(),
            code: String::new(),
            name: String::new(),
            login: None,
            password: None,
            main_role_id: None,
            roles_ids: None,
            main_role_code: None,
            role_codes: None,
            phone: None,
            cell_phone: None,
            first_name: None,
            middle_name: None,
            last_name: None,
            birthday: None,
            email: None,
            address: None,
            hire_date: None,
            hire_document_number: None,
            fire_date: None,
            note: None,
            card_number: None,
            pin_code: None,
            taxpayer_id_number: None,
            snils: None,
            gln: None,
            activation_date: None,
            deactivation_date: None,
            preferred_department_code: None,
            department_codes: None,
            responsibility_department_codes: None,
            deleted: None,
            supplier: false,
            employee: false,
            client: false,
            public_external_data: None,
        }
    }
}

/// Произвольные внешние данные (publicExternalData)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicExternalData {
    #[serde(rename = "entry", default)]
    pub entries: Vec<KeyValueEntry>,
}

/// Элемент произвольных внешних данных (entry)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValueEntry {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value", default)]
    pub value: Option<String>,
}

/// Список сотрудников (XML wrapper)
///
/// # Формат XML:
/// ```xml
/// <employees>
///   <employee>...</employee>
///   <employee>...</employee>
/// </employees>
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "employees")]
pub struct Employees {
    #[serde(rename = "employee", default)]
    pub items: Vec<Employee>,
}
