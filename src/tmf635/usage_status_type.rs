use serde::{Deserialize, Serialize};
///Possible values for the status of the Usage
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UsageStatusType {
    ///The usage is planned
    #[serde(rename = "received")]
    Received,
    ///The usage is accepted
    #[serde(rename = "rejected")]
    Rejected,
    ///The usage is in progress
    #[serde(rename = "recycled")]
    Recycled,
    ///The usage is completed
    #[serde(rename = "guided")]
    Guided,
    ///The usage is cancelled
    #[serde(rename = "rated")]
    Rated,
    ///The usage is rerated
    #[serde(rename = "rerated")]
    Rerated,
    ///The usage is billed
    #[serde(rename = "billed")]
    Billed,
}
