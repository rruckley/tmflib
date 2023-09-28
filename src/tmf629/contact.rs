//! Contact Module
//!
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// Contact Medium
#[derive(Clone, Default, Debug, Deserialize, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactMedium {
    characteristic: ContactCharacteristic,
    prefferred: bool,
}

impl ContactMedium {
    pub fn new() -> ContactMedium {
        ContactMedium {
            prefferred: false,
            characteristic: ContactCharacteristic {
                email_address: None,
                phone_number: None,
            },
        }
    }
}

/// Contact Characteristic
#[derive(Clone, Default, Debug, Deserialize, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactCharacteristic {
    email_address: Option<String>,
    phone_number: Option<String>,
}
