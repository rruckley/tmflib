use serde::{Serialize, Deserialize};
///Valid values for the lifecycle state of the service
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceStateType {
    ///The service is in the planning stage
    #[serde(rename = "feasibilityChecked")]
    FeasibilityChecked,
    ///The service is designed
    #[serde(rename = "designed")]
    Designed,
    ///The service is reserved
    #[serde(rename = "reserved")]
    Reserved,
    ///The service is active
    #[serde(rename = "inactive")]
    Inactive,
    ///The service is active
    #[serde(rename = "active")]
    Active,
    ///The service is terminated
    #[serde(rename = "terminated")]
    Terminated,
    ///The service is suspended
    #[serde(rename = "suspended")]
    Suspended,
}
