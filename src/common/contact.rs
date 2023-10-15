//! Contact Module
//!
use serde::{Deserialize, Serialize};
use std::hash::Hash;


/// Characteristics for contact mediums
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediumCharacteristic {
    email_address : Option<String>,
}

/// Contact Medium
#[derive(Clone, Default, Debug, Deserialize, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactMedium {
    characteristic: Option<MediumCharacteristic>,
    medium_type: Option<String>,
    preferred: bool,
}

impl ContactMedium {
    /// Create a new contact medium
    pub fn new() -> ContactMedium {
        ContactMedium {
            preferred: false,
            medium_type: None,
            characteristic: None,
        }
    }

    /// Generate a new contact medium representing email address
    /// # Example
    /// ```
    /// use tmflib::common::contact::ContactMedium;
    /// let medium = ContactMedium::email("john.smith@optus.com.au");
    /// ````
    pub fn email(email: &str) -> ContactMedium {
        let char = MediumCharacteristic {
            email_address : Some(email.to_string()),
        };
        ContactMedium {
            preferred : false,
            medium_type : Some(String::from("email")),
            characteristic : Some(char),
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
