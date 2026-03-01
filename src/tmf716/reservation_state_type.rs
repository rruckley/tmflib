use serde::{Serialize, Deserialize};
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReservationStateType {
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
}
