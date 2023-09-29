//! Service Order Management Module

use serde::{Deserialize,Serialize};

use super::service::Service;

#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct TMF638ServiceInventoryManagement {
    services : Vec<Service>,
}

impl TMF638ServiceInventoryManagement {
    pub fn new() -> TMF638ServiceInventoryManagement {
        TMF638ServiceInventoryManagement { services: vec![] }
    }
}