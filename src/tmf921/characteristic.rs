//! Intent Characteristic Module
//!

use serde::{Deserialize, Serialize};

/// Represents the value of a characteristic.
/// This enum is used to define the different types of values that a characteristic can have.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    /// A string array value.
    StringArrayCharacteristic(Vec<String>),
    /// An integer array value.
    IntegerArrayCharacteristic(Vec<i32>),
    /// A string value.
    StringCharacteristic(String),
    /// A float value.
    NumberCharacteristic(f32),
    /// A boolean value.
    BooleanCharacteristic(bool),
    /// A number array value.
    NumberArrayCharacteristic(Vec<f32>),
    /// A boolean value.
    IntegerCharacteristic(i32),
}

/// Represents a characteristic of an intent.
/// This struct is used to define the characteristics of an intent in the TMF921 API.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Characteristic {
    /// An optional unique identifier for the characteristic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// An optional name for the characteristic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An optional description of the characteristic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}
