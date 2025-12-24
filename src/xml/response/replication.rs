use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Тип сервера
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServerType {
    Chain,
    ReplicatedRms,
    StandaloneRms,
}

/// Статус репликации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStatus {
    #[serde(rename = "departmentId", default)]
    pub department_id: Option<Uuid>,
    #[serde(rename = "departmentName", default)]
    pub department_name: Option<String>,
    #[serde(rename = "lastReplicationDate", default)]
    pub last_replication_date: Option<String>,
    #[serde(rename = "status", default)]
    pub status: Option<String>,
    #[serde(rename = "errorMessage", default)]
    pub error_message: Option<String>,
}

/// Список статусов репликации
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "replicationStatuses")]
pub struct ReplicationStatuses {
    #[serde(rename = "replicationStatus", default)]
    pub items: Vec<ReplicationStatus>,
}

