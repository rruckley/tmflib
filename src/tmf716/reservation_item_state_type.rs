use serde::{Deserialize, Serialize};

/// The state of the reservation, such as 'reserved', 'completed', 'cancelled'
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReservationItemStateType {
    ///The reservation item has been acknowledged by the system.
    #[serde(rename = "acknowledged")]
    Acknowledged,
    ///The reservation item has been rejected by the system.
    #[serde(rename = "rejected")]
    Rejected,
    ///The reservation item is pending and waiting for processing.
    #[serde(rename = "pending")]
    Pending,
    ///The reservation item is reserved and waiting for the start date.
    #[serde(rename = "held")]
    Held,
    ///The reservation item is active and the resource is reserved.
    #[serde(rename = "inProgress")]
    InProgress,
    ///The reservation item is released and the resource is released.
    #[serde(rename = "cancelled")]
    Cancelled,
    ///The reservation item is released and the resource is released.
    #[serde(rename = "completed")]
    Completed,
    ///The reservation item is released and the resource is released.
    #[serde(rename = "failed")]
    Failed,
    ///The reservation item is released and the resource is released.
    #[serde(rename = "partial")]
    Partial,
    ///The reservation item is released and the resource is released.
    #[serde(rename = "assessingCancellation")]
    AssessingCancellation,
    ///The reservation item is released and the resource is released.
    #[serde(rename = "pendingCancellation")]
    PendingCancellation,
}
