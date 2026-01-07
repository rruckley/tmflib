use serde::{Serialize, Deserialize};
///Possible values for the status of the Usage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UsageStatusType {
    #[serde(rename = "received")]
    Received,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "recycled")]
    Recycled,
    #[serde(rename = "guided")]
    Guided,
    #[serde(rename = "rated")]
    Rated,
    #[serde(rename = "rerated")]
    Rerated,
    #[serde(rename = "billed")]
    Billed,
}
