//! Characteristic Module

use serde::{Deserialize, Serialize};
use crate::LIB_PATH;

const CHAR_PATH : &str = "characteristic";


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
    pub fn new(name: String) -> Characteristic {
        Characteristic { name, ..Default::default() }
    }
}