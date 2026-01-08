use serde::{Serialize, Deserialize};
///action to be performed on the product
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderItemActionType {
    ///Add a new product
    #[serde(rename = "add")]
    Add,
    ///Modify an existing product
    #[serde(rename = "modify")]
    Modify,
    ///Delete an existing product
    #[serde(rename = "delete")]
    Delete,
    ///No change to the product
    #[serde(rename = "noChange")]
    NoChange,
}
