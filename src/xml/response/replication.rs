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
#[derive(Debug, Clone, Serialize)]
pub struct ReplicationStatus {
    #[serde(rename = "departmentId", default)]
    pub department_id: Option<Uuid>,
    #[serde(rename = "departmentName", default)]
    pub department_name: Option<String>,
    /// Last successful receive from the chain. The iiko Server API uses
    /// `lastReceiveDate`; older installations returned the legacy
    /// `lastReplicationDate`, so keep that alias for compatibility.
    #[serde(rename = "lastReceiveDate", alias = "lastReplicationDate", default)]
    pub last_receive_date: Option<String>,
    /// Last successful send to the chain.
    #[serde(rename = "lastSendDate", default)]
    pub last_send_date: Option<String>,
    /// Legacy mirror retained for SDK compatibility. New callers should use
    /// `last_receive_date` and `last_send_date`.
    #[serde(rename = "lastReplicationDate", default)]
    pub last_replication_date: Option<String>,
    #[serde(rename = "status", default)]
    pub status: Option<String>,
    #[serde(rename = "errorMessage", default)]
    pub error_message: Option<String>,
}

#[derive(Deserialize)]
struct ReplicationStatusWire {
    #[serde(rename = "departmentId", default)]
    department_id: Option<Uuid>,
    #[serde(rename = "departmentName", default)]
    department_name: Option<String>,
    #[serde(rename = "lastReceiveDate", alias = "lastReplicationDate", default)]
    last_receive_date: Option<String>,
    #[serde(rename = "lastSendDate", default)]
    last_send_date: Option<String>,
    #[serde(rename = "status", default)]
    status: Option<String>,
    #[serde(rename = "errorMessage", default)]
    error_message: Option<String>,
}

impl<'de> Deserialize<'de> for ReplicationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let wire = ReplicationStatusWire::deserialize(deserializer)?;
        Ok(Self {
            department_id: wire.department_id,
            department_name: wire.department_name,
            last_replication_date: wire.last_receive_date.clone(),
            last_receive_date: wire.last_receive_date,
            last_send_date: wire.last_send_date,
            status: wire.status,
            error_message: wire.error_message,
        })
    }
}

/// Список статусов репликации
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "replicationStatuses")]
pub struct ReplicationStatuses {
    #[serde(rename = "replicationStatus", default)]
    pub items: Vec<ReplicationStatus>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;

    #[test]
    fn mirrors_current_receive_date_into_legacy_field() {
        let status: ReplicationStatus = from_str(
            "<replicationStatus><lastReceiveDate>2026-08-09T10:00:00</lastReceiveDate><lastSendDate>2026-08-09T09:59:00</lastSendDate></replicationStatus>",
        )
        .unwrap();

        assert_eq!(status.last_receive_date, status.last_replication_date);
        assert_eq!(
            status.last_send_date.as_deref(),
            Some("2026-08-09T09:59:00")
        );
    }

    #[test]
    fn accepts_legacy_replication_date() {
        let status: ReplicationStatus = from_str(
            "<replicationStatus><lastReplicationDate>2026-08-09T10:00:00</lastReplicationDate></replicationStatus>",
        )
        .unwrap();

        assert_eq!(
            status.last_receive_date.as_deref(),
            Some("2026-08-09T10:00:00")
        );
        assert_eq!(status.last_receive_date, status.last_replication_date);
    }
}
