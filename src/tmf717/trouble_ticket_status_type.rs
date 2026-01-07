use serde::{Serialize, Deserialize};
///Possible values for the status of the trouble ticket
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TroubleTicketStatusType {
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
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "resolved")]
    Resolved,
}
