//! Risk Assessment Characteristic Module

use crate::Uri;
use serde::{Deserialize,Serialize};

/// Risk Assessement Characteristic
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct Characteristic {
    id: Option<String>,
    /// Characteristic Name
    pub name: String,
    /// Characteristic Value
    pub value: String,
    value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    base_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    schema_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    r#type: Option<String>,
}

impl Characteristic {
    /// Create a new characteristic
    pub fn new(name: String,value: String) -> Characteristic {
        Characteristic {
            name,
            value,
            value_type: "String".to_string(),
            ..Default::default()
        }
    }
}


/// Risk Assessement Characteristic Relationship
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct CharacteristicRelationship {
    href: Option<Uri>,
    id: Option<String>,
    relationship_type: Option<String>,
}