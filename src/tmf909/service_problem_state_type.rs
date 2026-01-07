use serde::{Serialize, Deserialize};
///Possible values for the state of the ServiceProblem
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceProblemStateType {
    ///The service problem state is acknowledged
    #[serde(rename = "acknowledged")]
    Acknowledged,
    ///The service problem state is rejected
    #[serde(rename = "rejected")]
    Rejected,
    ///The service problem state is pending
    #[serde(rename = "pending")]
    Pending,
    ///The service problem state is held
    #[serde(rename = "held")]
    Held,
    ///The service problem state is in progress
    #[serde(rename = "inProgress")]
    InProgress,
    ///The service problem state is resolved
    #[serde(rename = "resolved")]
    Resolved,
    ///The service problem state is cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    ///The service problem state is closed
    #[serde(rename = "closed")]
    Closed,
}
