use serde::{Serialize, Deserialize};
///Valid values for the lifecycle state of the service
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceStateType {
    ///The service state is feasibilityChecked
    #[serde(rename = "feasibilityChecked")]
    FeasibilityChecked,
    ///The service state is designed
    #[serde(rename = "designed")]
    Designed,
    ///The service state is reserved
    #[serde(rename = "reserved")]
    Reserved,
    ///The service state is provisioned
    #[serde(rename = "inactive")]
    Inactive,
    ///The service state is active
    #[serde(rename = "active")]
    Active,
    ///The service state is terminated
    #[serde(rename = "terminated")]
    Terminated,
}
