use serde::{Deserialize, Serialize};
///`ResourceStatusType` enumerations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceStatusType {
    ///The resource is in a standby state and cannot be used.
    #[serde(rename = "standby")]
    Standby,
    ///The resource is in an alarm state and cannot be used.
    #[serde(rename = "alarm")]
    Alarm,
    ///The resource is in an available state and can be used.
    #[serde(rename = "available")]
    Available,
    ///The resource is in a reserved state and cannot be used.
    #[serde(rename = "reserved")]
    Reserved,
    ///The resource is in an error state and cannot be used.    
    #[serde(rename = "unknown")]
    Unknown,
    ///The resource is in a suspended state and cannot be used.
    #[serde(rename = "suspended")]
    Suspended,
}
