//!
use serde::{Deserialize, Serialize};

use super::product::Product;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TMF637ProductInventory {
    inventory: Vec<Product>,
}

impl TMF637ProductInventory {
    pub fn new() -> TMF637ProductInventory {
        let vec: Vec<Product> = vec![];
        TMF637ProductInventory { inventory: vec }
    }
}
