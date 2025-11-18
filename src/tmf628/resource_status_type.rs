use serde::{Serialize, Deserialize};
///ResourceStatusType enumerations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceStatusType {
    #[serde(rename = "alarm")]
    Alarm,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "not exists")]
    NotExists,
    #[serde(rename = "pendingRemoval")]
    PendingRemoval,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "standby")]
    Standby,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "unknown")]
    Unknown,
}
