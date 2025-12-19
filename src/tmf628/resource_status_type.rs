use serde::{Deserialize, Serialize};
///ResourceStatusType enumerations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceStatusType {
    ///The resource is in alarm state
    #[serde(rename = "alarm")]
    Alarm,
    ///The resource is available
    #[serde(rename = "available")]
    Available,
    ///The resource is being installed
    #[serde(rename = "installed")]
    Installed,
    ///The resource does not exist
    #[serde(rename = "not exists")]
    NotExists,
    ///The resource is operational
    #[serde(rename = "pendingRemoval")]
    PendingRemoval,
    ///The resource is planned
    #[serde(rename = "planned")]
    Planned,
    ///The resource is reserved
    #[serde(rename = "reserved")]
    Reserved,
    ///The resource is in standby mode
    #[serde(rename = "standby")]
    Standby,
    ///The resource is suspended
    #[serde(rename = "suspended")]
    Suspended,
    ///The resource is unknown
    #[serde(rename = "unknown")]
    Unknown,
}
