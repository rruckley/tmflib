//! Customer Characteristic Module

use serde::{Deserialize, Serialize};

/// Customer Characteristics
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    /// Name of characteristic
    pub name: String,
    /// Type of characteristic
    pub value_type: String,
    /// Value of characteristic
    pub value: String,
}

impl Characteristic {
    /// Create a new characteristic from name / value
    pub fn new(name : impl Into<String>, value : impl Into<String>) -> Characteristic {
        Characteristic {
            name : name.into(),
            value: value.into(),
            value_type : String::from("String"),
        }
    }
}

impl From<(&str, &str)> for Characteristic {
    fn from(value: (&str, &str)) -> Self {
        let (name,value) = value;
        Characteristic::new(name,value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const CHAR_JSON : &str = "{
        \"name\" : \"ABN\",
        \"valueType\" : \"string\",
        \"value\" : \"123.456.789\"
    }";

    #[test]
    fn test_characteristic_deserialize() {
        let characteristic : Characteristic = serde_json::from_str(CHAR_JSON)
            .expect("Could not parse CHAR_JSON");

        assert_eq!(characteristic.name.as_str(),"ABN");
        assert_eq!(characteristic.value_type.as_str(),"string");
        assert_eq!(characteristic.value.as_str(),"123.456.789");
    }
}
