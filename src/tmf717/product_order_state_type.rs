use serde::{Serialize, Deserialize};
///Possible values for the state of the order
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProductOrderStateType {
    ///Order has been acknowledged
    #[serde(rename = "acknowledged")]
    Acknowledged,
    ///Order has been accepted
    #[serde(rename = "rejected")]
    Rejected,
    ///Order is pending
    #[serde(rename = "pending")]
    Pending,
    ///Order is held
    #[serde(rename = "held")]
    Held,
    ///Order is in progress
    #[serde(rename = "inProgress")]
    InProgress,
    ///Order has been completed
    #[serde(rename = "cancelled")]
    Cancelled,
    ///Order has been completed
    #[serde(rename = "completed")]
    Completed,
    ///Order has failed
    #[serde(rename = "failed")]
    Failed,
    ///Order is partially completed
    #[serde(rename = "partial")]
    Partial,
    ///Order cancellation is being assessed
    #[serde(rename = "assessingCancellation")]
    AssessingCancellation,
    ///Order cancellation is pending
    #[serde(rename = "pendingCancellation")]
    PendingCancellation,
    ///Order is in draft state
    #[serde(rename = "draft")]
    Draft,
    ///Order is accepted and in progress
    #[serde(rename = "inProgress.accepted")]
    InProgressAccepted,
}
