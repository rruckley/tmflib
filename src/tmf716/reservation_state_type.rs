use serde::{Deserialize, Serialize};

/// The state of the reservation, such as 'reserved', 'completed', 'cancelled'
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReservationStateType {
    ///The reservation is in an accepted state and is waiting to be processed.
    #[serde(rename = "acknowledged")]
    Acknowledged,
    ///The reservation is in a rejected state and will not be processed.
    #[serde(rename = "rejected")]
    Rejected,
    ///The reservation is in a pending state and is waiting to be processed.
    #[serde(rename = "pending")]
    Pending,
    ///The reservation is in a held state and is waiting to be processed.
    #[serde(rename = "held")]
    Held,
    ///The reservation is in an in progress state and is being processed.
    #[serde(rename = "inProgress")]
    InProgress,
    ///The reservation is in a cancelled state and will not be processed.
    #[serde(rename = "cancelled")]
    Cancelled,
    ///The reservation is in a completed state and has been processed.
    #[serde(rename = "completed")]
    Completed,
    ///The reservation is in a failed state and has been processed with errors.
    #[serde(rename = "failed")]
    Failed,
    ///The reservation is in a partial state and has been processed with some errors.
    #[serde(rename = "partial")]
    Partial,
    ///The reservation is in an assessing cancellation state and is being assessed for cancellation.
    #[serde(rename = "assessingCancellation")]
    AssessingCancellation,
    ///The reservation is in a pending cancellation state and is waiting for cancellation to be processed.
    #[serde(rename = "pendingCancellation")]
    PendingCancellation,
}
