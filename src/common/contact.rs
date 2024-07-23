//! Contact Module
//!
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::TimePeriod;
use super::related_party::RelatedParty;

const EMAIL_TYPE : &str = "email";

/// Characteristics for contact mediums
#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediumCharacteristic {
    /// Contact Type
    pub contact_type: Option<String>,
    /// Contact Email Address
    pub email_address : Option<String>,
    /// Contact Phone Number
    pub phone_number: Option<String>,
}

/// Contact Medium
#[derive(Clone, Default, Debug, Deserialize, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactMedium {
    /// Characteristic
    pub characteristic: Option<MediumCharacteristic>,
    /// Medium Type, e.g. Mobile, Email etc.
    pub medium_type: Option<String>,
    /// Is this the preferred medium
    pub preferred: bool,
}

impl ContactMedium {
    /// Create a new contact medium
    pub fn new() -> ContactMedium {
        ContactMedium::default()
    }

    /// Generate a new contact medium representing email address
    /// # Example
    /// ```
    /// use tmflib::common::contact::ContactMedium;
    /// let medium = ContactMedium::email("john.smith@example.com");
    /// ````
    pub fn email(email: &str) -> ContactMedium {
        let char = MediumCharacteristic {
            email_address : Some(email.to_string()),
            contact_type: Some(String::from(EMAIL_TYPE)),
            phone_number: None,
        };
        ContactMedium {
            preferred : false,
            medium_type : Some(String::from(EMAIL_TYPE)),
            characteristic : Some(char),
        }
    }

    /// Generate a new contact medium representing a mobile phone number
    /// # Example
    /// ```
    /// use tmflib::common::contact::ContactMedium;
    /// let medium = ContactMedium::mobile("0411 111 111");
    /// ```
    pub fn mobile(mobile : &str) -> ContactMedium {
        let char = MediumCharacteristic {
            email_address : None,
            contact_type: Some(String::from("mobile")),
            phone_number: Some(mobile.to_string()),
        };
        ContactMedium { 
            characteristic: Some(char), 
            medium_type: Some(String::from("phone")), 
            preferred: false,
        }
    }
}

/// A singular contact
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Contact {
    contact_name: String,
    contact_type: String,
    party_role_type: String,
    valid_for: Option<TimePeriod>,
    related_party: Option<RelatedParty>,
    contact_medium: Option<Vec<ContactMedium>>,
}

/// Contact Characteristic
#[derive(Clone, Default, Debug, Deserialize, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactCharacteristic {
    email_address: Option<String>,
    phone_number: Option<String>,
}

#[cfg(test)]
mod test {
    use super::ContactMedium;
    use super::EMAIL_TYPE;

    #[test]
    fn test_contact_new() {
        let contact = ContactMedium::new();

        assert_eq!(contact.medium_type,None);
    }

    #[test]
    fn test_contact_email() {
        let email = ContactMedium::email("test@example.com");

        assert_eq!(email.medium_type.unwrap(),EMAIL_TYPE.to_string());
        assert_eq!(email.characteristic.is_some(),true);
        assert_eq!(email.characteristic.unwrap().contact_type.unwrap(),EMAIL_TYPE.to_string());
        //assert_eq!(email.characteristic.unwrap().email_address.unwrap(),"test@example.com".to_string());
    }
}
