use serde::{Serialize, Deserialize};
///Valid values for the lifecycle state of the service
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceStateType {
    #[serde(rename = "feasibilityChecked")]
    FeasibilityChecked,
    #[serde(rename = "designed")]
    Designed,
    #[serde(rename = "reserved")]
    Reserved,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "terminated")]
    Terminated,
    #[serde(rename = "suspended")]
    Suspended,
}
