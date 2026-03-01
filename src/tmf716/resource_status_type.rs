use serde::{Serialize, Deserialize};
///ResourceStatusType enumerations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResourceStatusType {
    #[serde(rename = "standby")]
    Standby,
    #[serde(rename = "alarm")]
    Alarm,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "suspended")]
    Suspended,
}
