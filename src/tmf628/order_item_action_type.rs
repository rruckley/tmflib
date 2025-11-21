use serde::{Serialize, Deserialize};
///action to be performed on the product
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderItemActionType {
    /// Add a product to the order
    #[serde(rename = "add")]
    Add,
    /// Modify a product in the order
    #[serde(rename = "modify")]
    Modify,
    /// Delete a product from the order
    #[serde(rename = "delete")]
    Delete,
    /// No change to the product in the order
    #[serde(rename = "noChange")]
    NoChange,
}
