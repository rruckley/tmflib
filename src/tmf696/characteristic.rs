//! Risk Assessment Characteristic Module

use crate::Uri;
use serde::{Deserialize,Serialize};

/// Risk Assessement Characteristic
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
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
    #[serde(rename = "@schemaLocation")]
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

    const CHARREL_JSON : &str = "{
        \"id\" : \"CR123\"
    }";
    const CHAR_JSON : &str = "{
        \"name\" : \"CharacteristicName\",
        \"value\" : \"CharacteristicValue\",
        \"valueType\" : \"string\"
    }";


    #[test]
    fn test_characteristic_new() {
        let characteristic = Characteristic::new(CHAR_NAME,CHAR_VALUE);

        assert_eq!(characteristic.name,CHAR_NAME.to_string());
        assert_eq!(characteristic.value,CHAR_VALUE.to_string());
        assert_eq!(characteristic.value_type,"String".to_string());
    }

    #[test]
    fn test_charrel_deserialize() {
        let charrel : CharacteristicRelationship = serde_json::from_str(CHARREL_JSON).unwrap();

        assert_eq!(charrel.id.is_some(),true);
        assert_eq!(charrel.id.unwrap().as_str(),"CR123");
    }

    #[test]
    fn test_char_deserialize() {
        let characteristic : Characteristic = serde_json::from_str(CHAR_JSON).unwrap();

        assert_eq!(characteristic.name.as_str(),"CharacteristicName");
        assert_eq!(characteristic.value.as_str(),"CharacteristicValue");
        assert_eq!(characteristic.value_type.as_str(),"string");
    }
}