use serde::{Deserialize, Serialize};
///Possible values for the state of the execution
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExecutionStateType {
    ///The execution is acknowledged
    #[serde(rename = "acknowledged")]
    Acknowledged,
    ///The execution is rejected
    #[serde(rename = "rejected")]
    Rejected,
    ///The execution is pending
    #[serde(rename = "pending")]
    Pending,
    ///The execution is in progress
    #[serde(rename = "inProgress")]
    InProgress,
    ///The execution is cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    ///The execution is completed
    #[serde(rename = "completed")]
    Completed,
    ///The execution has failed
    #[serde(rename = "failed")]
    Failed,
}
