//! Customer Characteristic Module

use serde::{Deserialize, Serialize};

/// Customer Characteristic
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Characteristic {
    pub name: String,
    pub value_type: String,
    pub value: String,
}
