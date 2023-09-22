//! Service Order Management Module
//! 

use super::service_order::ServiceOrder;

use serde::{Deserialize,Serialize};

/// Service Order Management
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct TMF641ServiceOrderManagement {
    orders  : Vec<ServiceOrder>,
}