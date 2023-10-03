//!
use serde::{Deserialize, Serialize};

use super::product::Product;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TMF637ProductInventory {
    products: Vec<Product>,
}

impl TMF637ProductInventory {
    pub fn new() -> TMF637ProductInventory {
        TMF637ProductInventory { products: vec![] }
    }

    pub fn add_product(&mut self, product : Product) {
        self.products.push(product);
    }
}
