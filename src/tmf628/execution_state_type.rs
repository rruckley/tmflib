use serde::{Serialize, Deserialize};
///Possible values for the state of the execution
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExecutionStateType {
    #[serde(rename = "acknowledged")]
    Acknowledged,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
