//! Work Order Item Module
//! 
//! 

use serde::{Deserialize,Serialize};

/// Work Order Item
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct WorkOrderItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    base_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    schema_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}