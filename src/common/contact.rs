//! Contact Module
//!
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::{HasName, TimePeriod};
use super::related_party::RelatedParty;
#[cfg(feature = "tmf632-v4")]
use crate::tmf632::individual_v4::Individual;
#[cfg(feature = "tmf632-v5")]
use crate::tmf632::individual_v5::Individual;

const EMAIL_TYPE : &str = "email";
const MOBILE_TYPE: &str = "mobile";

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
            contact_type: Some(String::from(MOBILE_TYPE)),
            phone_number: Some(mobile.to_string()),
        };
        ContactMedium { 
            characteristic: Some(char), 
            medium_type: Some(String::from(MOBILE_TYPE)), 
            preferred: false,
        }
    }
}

/// A singular contact
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    /// Name of contact
    pub contact_name: String,
    /// Type of contact, primary, secondary
    contact_type: String,
    /// Related TMF role, e.g. individual, organization etc.
    party_role_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<RelatedParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_medium: Option<Vec<ContactMedium>>,
}

impl Contact {
    /// Create a new contact
    pub fn new(name : impl Into<String>) -> Contact {
        Contact {
            contact_name: name.into(),
            ..Default::default()
        }
    }
}

impl From<&Individual> for Contact {
    fn from(value: &Individual) -> Self {
        let contact = Contact::new(value.get_name());
        contact
    }
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
    use crate::tmf632::individual_v4::Individual;

    use super::Contact;
    use super::ContactCharacteristic;
    use super::ContactMedium;
    use super::MediumCharacteristic;
    use super::EMAIL_TYPE;
    use super::MOBILE_TYPE;
    use crate::HasName;

    const CONTACT_JSON : &str = "{
        \"contactName\" :\"John Quinton Citizen\",
        \"contactType\" :\"primary\",
        \"partyRoleType\":\"individual\"
    }";

    const MEDIUM_CHAR_JSON : &str = "{
        \"contactType\" : \"email\",
        \"emailAddress\": \"john@example.com\"
    }";

    const CONTACT_MEDIUM: &str = "{
        \"preferred\" : true
    }";

    const CONTACT_CHAR : &str = "{
        \"emailAddress\" : \"john@example.com\"
    }";

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
    }

    #[test]
    fn test_contact_mobile() {
        let mobile = ContactMedium::mobile("0411111111");

        assert_eq!(mobile.medium_type.unwrap(),MOBILE_TYPE.to_string());
        assert_eq!(mobile.characteristic.is_some(),true);
        assert_eq!(mobile.characteristic.unwrap().contact_type.unwrap(),MOBILE_TYPE.to_string());   
    }

    #[test]
    fn test_contact_from_individual() {
        let individual = Individual::new("John Quinton Citizen");
        let contact = Contact::from(&individual);

        assert_eq!(individual.get_name(), contact.contact_name);
    }

    #[test]
    fn test_mediumcharacteristic_deserialize() {
        let mediumchar: MediumCharacteristic = serde_json::from_str(MEDIUM_CHAR_JSON).unwrap();

        assert_eq!(mediumchar.contact_type.is_some(),true);
        assert_eq!(mediumchar.contact_type.expect("Could not parse mediumchar JSON").as_str(),"email");

        assert_eq!(mediumchar.email_address.is_some(),true);
        assert_eq!(mediumchar.email_address.expect("Could not parse email_address JSON").as_str(),"john@example.com");
    }

    #[test]
    fn test_contact_deserialize() {
        let contact : Contact = serde_json::from_str(CONTACT_JSON).unwrap();

        assert_eq!(contact.contact_name.as_str(),"John Quinton Citizen");
        assert_eq!(contact.contact_type.as_str(),"primary");
        assert_eq!(contact.party_role_type.as_str(),"individual");
    }

    
    #[test]
    fn test_contactmedium_deserialize() {
        let contact_medium : ContactMedium = serde_json::from_str(CONTACT_MEDIUM).unwrap();

        assert_eq!(contact_medium.preferred,true);
    }

    #[test]
    fn test_contactcharacteristic_deserialize() {
        let contact_char : ContactCharacteristic = serde_json::from_str(CONTACT_CHAR).unwrap();

        assert_eq!(contact_char.email_address.is_some(),true);
        assert_eq!(contact_char.email_address.expect("Could not parse email_address JSON").as_str(),"john@example.com");
    }
}
