//! Service Order Management Module

use serde::{Deserialize,Serialize};

use super::service::Service;

/// Service Inventory Management layer
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct TMF638ServiceInventoryManagement {
    services : Vec<Service>,
}

impl TMF638ServiceInventoryManagement {
    /// Create a new instance of the inventory mangement struct
    pub fn new() -> TMF638ServiceInventoryManagement {
        TMF638ServiceInventoryManagement { services: vec![] }
    }
}