use serde::{Serialize, Deserialize};
///Possible values for the status of the trouble ticket
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TroubleTicketStatusType {
    ///Trouble ticket has been acknowledged
    #[serde(rename = "acknowledged")]
    Acknowledged,
    ///Trouble ticket has been rejected
    #[serde(rename = "rejected")]
    Rejected,
    ///Trouble ticket is pending
    #[serde(rename = "pending")]
    Pending,
    ///Trouble ticket is held
    #[serde(rename = "held")]
    Held,
    ///Trouble ticket is in progress
    #[serde(rename = "inProgress")]
    InProgress,
    ///Trouble ticket has been cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    ///Trouble ticket has been closed
    #[serde(rename = "closed")]
    Closed,
    ///Trouble ticket has been resolved
    #[serde(rename = "resolved")]
    Resolved,
}
