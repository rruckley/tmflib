use serde::{Serialize, Deserialize};
///action to be performed on the product
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderItemActionType {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "modify")]
    Modify,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "noChange")]
    NoChange,
}
