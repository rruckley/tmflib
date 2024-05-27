//! Work Order Item Module
//! 
//! 

use serde::{Deserialize,Serialize};

/// Work Order Item
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct WorkOrderItem {
    r#type: Option<String>,
    base_type: Option<String>,
    schema_location: Option<String>,
    id: Option<String>,
}