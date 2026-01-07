use serde::{Serialize, Deserialize};
///Possible values for the state of the quote
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QuoteStateType {
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
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "declined")]
    Declined,
}
