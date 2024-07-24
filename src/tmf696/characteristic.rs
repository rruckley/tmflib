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
    pub fn new(name: impl Into<String>,value: impl Into<String>) -> Characteristic {
        Characteristic {
            name : name.into(),
            value : value.into(),
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

#[cfg(test)]
mod test {
    use super::*;

    const CHAR_NAME : &str = "A Characteristic";
    const CHAR_VALUE: &str = "A Value";

    #[test]
    fn test_characteristic_new() {
        let characteristic = Characteristic::new(CHAR_NAME,CHAR_VALUE);

        assert_eq!(characteristic.name,CHAR_NAME.to_string());
        assert_eq!(characteristic.value,CHAR_VALUE.to_string());
        assert_eq!(characteristic.value_type,"String".to_string());
    }
}