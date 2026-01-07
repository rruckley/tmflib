use serde::{Serialize, Deserialize};
///Possible values for the state of the quote
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QuoteStateType {
    ///Quote has been acknowledged
    #[serde(rename = "acknowledged")]
    Acknowledged,
    ///Quote has been rejected
    #[serde(rename = "rejected")]
    Rejected,
    ///Quote is pending
    #[serde(rename = "pending")]
    Pending,
    ///Quote is held
    #[serde(rename = "inProgress")]
    InProgress,
    ///Quote has been cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    ///Quote has been approved
    #[serde(rename = "approved")]
    Approved,
    ///Quote has been accepted
    #[serde(rename = "accepted")]
    Accepted,
    ///Quote is declined
    #[serde(rename = "declined")]
    Declined,
}
