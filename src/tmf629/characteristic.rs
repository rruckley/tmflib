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
