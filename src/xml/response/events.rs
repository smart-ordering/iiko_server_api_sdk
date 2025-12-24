use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Список событий
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "eventsList")]
pub struct EventsList {
    #[serde(rename = "event", default)]
    pub events: Vec<Event>,
    #[serde(rename = "revision", default)]
    pub revision: Option<i64>,
}

/// Событие
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Уникальный идентификатор события (guid)
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Дата и время события
    #[serde(rename = "date", default)]
    pub date: Option<String>,
    /// Тип события
    #[serde(rename = "type", default)]
    pub r#type: Option<String>,
    /// ID департамента (guid) - для iikoChain
    #[serde(rename = "departmentId", default)]
    pub department_id: Option<Uuid>,
    /// Список атрибутов события
    #[serde(rename = "attribute", default)]
    pub attributes: Vec<EventAttribute>,
}

/// Атрибут события
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttribute {
    /// Имя атрибута
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// Значение атрибута
    #[serde(rename = "value", default)]
    pub value: Option<String>,
    /// Тип атрибута (java.lang.Boolean, java.lang.String, java.util.Date, resto.db.Guid, User, Department, Terminal, и т.д.)
    #[serde(rename = "type", default)]
    pub r#type: Option<String>,
}

/// Дерево событий (метаданные)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "groupsList")]
pub struct GroupsList {
    #[serde(rename = "group", default)]
    pub groups: Vec<EventGroup>,
}

/// Группа событий
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGroup {
    /// Идентификатор группы
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Имя группы
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// Список событий, входящих в данную группу
    #[serde(rename = "type", default)]
    pub types: Vec<EventType>,
}

/// Тип события (метаданные)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventType {
    /// Идентификатор события
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Имя события
    #[serde(rename = "name", default)]
    pub name: Option<String>,
    /// Важность события (0 - Низкая, 1 - Средняя, 2 - Высокая)
    #[serde(rename = "severity", default)]
    pub severity: Option<String>,
    /// Список атрибутов события
    #[serde(rename = "attribute", default)]
    pub attributes: Vec<EventTypeAttribute>,
}

/// Атрибут типа события (метаданные)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTypeAttribute {
    /// Идентификатор атрибута
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    /// Имя атрибута
    #[serde(rename = "name", default)]
    pub name: Option<String>,
}

/// Информация о кассовой смене
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashSession {
    #[serde(rename = "id", default)]
    pub id: Option<Uuid>,
    #[serde(rename = "openTime", default)]
    pub open_time: Option<String>,
    #[serde(rename = "closeTime", default)]
    pub close_time: Option<String>,
    #[serde(rename = "manager", default)]
    pub manager: Option<String>,
    #[serde(rename = "sessionNumber", default)]
    pub session_number: Option<String>,
    #[serde(rename = "cashRegisterNumber", default)]
    pub cash_register_number: Option<String>,
    #[serde(rename = "operDay", default)]
    pub oper_day: Option<String>,
}

/// Список кассовых смен
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashSessionsList {
    #[serde(rename = "session", default)]
    pub sessions: Vec<CashSession>,
}

