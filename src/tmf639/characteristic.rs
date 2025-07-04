//! Characteristic Module

use serde::{Deserialize, Serialize};

/// Resource Characteristic
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Characteristic {
    id: Option<String>,
    /// Characteristic Name
    pub name: String,
    value: Option<serde_json::Value>,
    value_type: Option<String>,
}

impl Characteristic {
    /// Create a new resource characteristic
    pub fn new(name: impl Into<String>) -> Characteristic {
        Characteristic { name : name.into(), ..Default::default() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const CHAR_NAME : &str = "CharacteristicName";
    const CHAR_JSON : &str = "{
        \"name\" : \"CharacteristicName\"
    }";
    #[test]
    fn test_characteristic_new() {
        let char = Characteristic::new(CHAR_NAME);

        assert_eq!(char.name.as_str(),CHAR_NAME);
    }

    #[test]
    fn test_characteristic_deserialize() {
        let char : Characteristic = serde_json::from_str(CHAR_JSON).unwrap();

        assert_eq!(char.name.as_str(),"CharacteristicName");
    }
}