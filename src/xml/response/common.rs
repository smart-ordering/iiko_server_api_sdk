use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Общая структура для XML ответов
#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "$value", default)]
    pub data: T,
}

/// Общая структура Id-Name для различных сущностей
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdName {
    #[serde(rename = "id")]
    pub id: Uuid,
    #[serde(rename = "name")]
    pub name: String,
}

