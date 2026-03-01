use serde::{Serialize, Deserialize};
///action to be performed on the entity
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReservationItemActionType {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "modify")]
    Modify,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "noChange")]
    NoChange,
}
