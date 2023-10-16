//!
use serde::{Deserialize, Serialize};

use super::product::Product;

/// Product Inventory management layer
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TMF637ProductInventory {
    products: Vec<Product>,
}

impl TMF637ProductInventory {
    /// Create a new inventory instance
    /// # Warning
    /// This will be moved into platypus
    pub fn new() -> TMF637ProductInventory {
        TMF637ProductInventory { products: vec![] }
    }

    /// Add a new product into the inventory
    /// # Warning
    /// This will bemoved into platypus
    pub fn add_product(&mut self, product : Product) {
        self.products.push(product);
    }
}
