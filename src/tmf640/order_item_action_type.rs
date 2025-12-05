use serde::{Serialize, Deserialize};
///action to be performed on the product
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderItemActionType {
    ///Add a product or service
    #[serde(rename = "add")]
    Add,
    ///Modify a product or service
    #[serde(rename = "modify")]
    Modify,
    ///Delete a product or service
    #[serde(rename = "delete")]
    Delete,
    ///No change to a product or service
    #[serde(rename = "noChange")]
    NoChange,
}
