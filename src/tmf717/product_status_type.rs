use serde::{Serialize, Deserialize};
///Possible values for the status of the product
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProductStatusType {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "pendingActive")]
    PendingActive,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pendingTerminate")]
    PendingTerminate,
    #[serde(rename = "terminated")]
    Terminated,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "aborted ")]
    Aborted,
}
