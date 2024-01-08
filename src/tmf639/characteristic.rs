//! Characteristic Module

use serde::{Deserialize, Serialize};

/// Resource Characteristic
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Characteristic {
    id: Option<String>,
    name: String,
    value: Option<String>,
    value_type: Option<String>,
}

impl Characteristic {
    /// Create a new resource characteristic
    pub fn new(name: impl Into<String>) -> Characteristic {
        Characteristic { name : name.into(), ..Default::default() }
    }
}