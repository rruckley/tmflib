use serde::{Serialize, Deserialize};
///Possible values for the state of the order item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceOrderItemStateType {
    ///The service order item state is acknowledged
    #[serde(rename = "acknowledged")]
    Acknowledged,
    ///The service order item state is rejected
    #[serde(rename = "rejected")]
    Rejected,
    ///The service order item state is pending
    #[serde(rename = "pending")]
    Pending,
    ///The service order item state is held
    #[serde(rename = "held")]
    Held,
    ///The service order item state is in progress
    #[serde(rename = "inProgress")]
    InProgress,
    ///The service order item state is cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    ///The service order item state is completed
    #[serde(rename = "completed")]
    Completed,
    ///The service order item state is failed
    #[serde(rename = "failed")]
    Failed,
    ///The service order item state is assessing cancellation
    #[serde(rename = "assessingCancellation")]
    AssessingCancellation,
    ///The service order item state is pending cancellation
    #[serde(rename = "pendingCancellation")]
    PendingCancellation,
    ///The service order item state is partial
    #[serde(rename = "partial")]
    Partial,
}
