use serde::{Serialize, Deserialize};
///Possible values for the status of the product
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProductStatusType {
    ///Product has been created
    #[serde(rename = "created")]
    Created,
    ///Product is pending activation
    #[serde(rename = "pendingActive")]
    PendingActive,
    ///Product has been cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    ///Product is active
    #[serde(rename = "active")]
    Active,
    ///Product is pending termination
    #[serde(rename = "pendingTerminate")]
    PendingTerminate,
    ///Product has been terminated
    #[serde(rename = "terminated")]
    Terminated,
    ///Product is suspended
    #[serde(rename = "suspended")]
    Suspended,
    ///Product has been aborted
    #[serde(rename = "aborted ")]
    Aborted,
}
