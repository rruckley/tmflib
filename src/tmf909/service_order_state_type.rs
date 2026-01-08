use serde::{Serialize, Deserialize};
///Possible values for the state of the order
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceOrderStateType {
    ///The service order state is acknowledged
    #[serde(rename = "acknowledged")]
    Acknowledged,
    ///The service order state is rejected
    #[serde(rename = "rejected")]
    Rejected,
    ///The service order state is pending
    #[serde(rename = "pending")]
    Pending,
    ///The service order state is held
    #[serde(rename = "held")]
    Held,
    ///The service order state is in progress
    #[serde(rename = "inProgress")]
    InProgress,
    ///The service order state is cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    ///The service order state is completed
    #[serde(rename = "completed")]
    Completed,
    ///The service order state is failed
    #[serde(rename = "failed")]
    Failed,
    ///The service order state is partial
    #[serde(rename = "partial")]
    Partial,
    ///The service order state is assessing cancellation
    #[serde(rename = "assessingCancellation")]
    AssessingCancellation,
    ///The service order state is pending cancellation
    #[serde(rename = "pendingCancellation")]
    PendingCancellation,
}
