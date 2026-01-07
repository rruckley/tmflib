use serde::{Serialize, Deserialize};
///Possible values for the state of the order
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProductOrderStateType {
    #[serde(rename = "acknowledged")]
    Acknowledged,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "held")]
    Held,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "assessingCancellation")]
    AssessingCancellation,
    #[serde(rename = "pendingCancellation")]
    PendingCancellation,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "inProgress.accepted")]
    InProgressAccepted,
}
