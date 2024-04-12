//! Customer Characteristic Module

use serde::{Deserialize, Serialize};

/// Customer Characteristics
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
