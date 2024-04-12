//! Customer Module
//!
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::CreateTMF;
use crate::tmf632::organization::Organization;

use super::characteristic::Characteristic;
use crate::common::contact::ContactMedium;
use crate::common::related_party::RelatedParty;
use crate::{HasId,HasName, HasValidity, TimePeriod};
use tmflib_derive::{HasId,HasName,HasValidity};

use crate::LIB_PATH;
use super::MOD_PATH;

const CLASS_PATH : &str = "customer";
const CUST_ID_SIZE : usize = 5;
/// Default customer status
pub const CUST_STATUS : &str = "New";

/// Customer object
#[derive(Clone, Default, Debug, Deserialize, HasId, HasName, HasValidity, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    /// Html Reference to this object
    pub href: Option<String>,
    /// Unique Id
    pub id: Option<String>,
    /// Name of this 
    pub name: Option<String>,
    /// Customer status
    pub status: Option<String>,
    /// Reason for current status
    pub status_reason: Option<String>,
    /// Validity of this record
    pub valid_for: Option<TimePeriod>,
    contact_medium: Option<Vec<ContactMedium>>,
    characteristic: Option<Vec<Characteristic>>,
    related_party: Vec<RelatedParty>,
    engaged_party: Option<RelatedParty>,
}

impl Customer {
    /// Create new customer object against an [Organization] (legal entity)
    /// ```
    /// # use tmflib::tmf629::customer::Customer;
    /// # use tmflib::tmf632::organization::Organization;
    /// let org = Organization::new("Legal Entity");
    /// let cust = Customer::new(org);
    /// ```
    pub fn new(org: Organization) -> Customer {
        let mut cust = Customer::create();
        cust.name = Some(org.get_name());
        // Not sure on including the name here but the id is only generated on create(), so a name change would
        // not impact the generated code. Ideally as we're throwing away a lot of the resulting hash to get the
        // code, it might help avoid collisions if we add some more entropy?
        cust.generate_code(None);
        cust.engaged_party = Some(RelatedParty::from(org));
        
        cust.status = Some(CUST_STATUS.to_string());
        cust.contact_medium = Some(vec![]);
        cust
    }

    /// Geneate a unique customer code via cryptographic functions
    pub fn generate_code(&mut self, offset : Option<u32>) {
        // Generate a new code based on name

        // Generate Id if none exists
        if self.id.is_none() {
            self.generate_id();
        };
        let offset = offset.unwrap_or(0);
        let hash_input = format!("{}:{}:{}", self.id.as_ref().unwrap(), self.get_name(),offset);
        let sha = digest(hash_input);
        let sha_slice = sha.as_str()[..CUST_ID_SIZE].to_string().to_ascii_uppercase();
        let code = Characteristic {
            name: String::from("code"),
            value_type: String::from("string"),
            value: sha_slice,
        };
        let hash = Characteristic {
            name: String::from("hash"),
            value_type: String::from("string"),
            value: sha,
        };
        // Create vec if it doesn't exist
        if self.characteristic.is_none() {
            self.characteristic = Some(vec![]);
        }

        // Replace characteristics if they exist, to ensure only a single instance of each
        self.replace_characteristic(code);
        self.replace_characteristic(hash);
    }

    /// Try to find characteristic with given name
    pub fn get_characteristic(&self, characteristic : &str) -> Option<Characteristic> {
        match &self.characteristic {
            Some(c) => {
                let found = c.into_iter().find(|x| x.name == characteristic);
                found.cloned()
            },
            None => None,
        }
    }

    /// Replace a characteristic returning the old value if found
    pub fn replace_characteristic(&mut self, characteristic : Characteristic) -> Option<Characteristic> {
        match self.characteristic.as_mut() {
            Some(c) => {
                // Characteristic array exist
                let pos = c.iter().position(|c| c.name == characteristic.name);
                match pos {
                    Some(u) => {
                        // Clone old value for return
                        let old = c[u].clone();
                        // Replace
                        c[u] = characteristic;
                        Some(old)
                    },
                    None => {
                        // This means the characteristic could not be found, instead we insert it
                        // Additional we return None to indicate that no old value was found
                        c.push(characteristic);
                        None
                    },
                }
            }
            None => {
                // Characteristic Vec was not created yet, create it now.
                self.characteristic = Some(vec![characteristic]);
                // Return None to show no previous value existed.
                None
            },
        }
    }

    /// Set the name of the customer
    pub fn name(&mut self, name : String) {
        self.name = Some(name.clone());
    }
}

impl From<&Organization> for Customer {
    fn from(value: &Organization) -> Self {
        let mut customer = Customer::new(value.to_owned());
        customer.generate_code(None);
        customer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const CUSTOMER : &str = "ACustomer";
    const CUSTOMER_BAD : &str = " ACustomer ";
    const CUSTOMER_UID : u16 = 174;

    #[test]
    fn test_customer_new_name() {
        let org = Organization::new(CUSTOMER);
        let customer = Customer::new(org);

        assert_eq!(customer.name, Some(CUSTOMER.into()));
        assert_eq!(customer.id.is_some(),true);
        assert_eq!(customer.href.is_some(),true);
    }

    #[test]
    fn test_customer_new_org() {
        let org1 = Organization::new(CUSTOMER);
        let org2 = org1.clone();
        let customer = Customer::new(org1);

        assert_eq!(customer.engaged_party,Some(RelatedParty::from(org2)));    
    }

    #[test]
    fn test_customer_new_code() {
        let org1 = Organization::new(CUSTOMER);
        let customer = Customer::new(org1);

        assert!(customer.get_characteristic("code").is_some());
    }

    #[test]
    fn test_customer_from_org() {
        let org1 = Organization::new(CUSTOMER);
        let customer = Customer::from(&org1);

        assert_eq!(org1.name,customer.name);
    }

    #[test]
    fn test_customer_characteristic_replace() {
        let org1 = Organization::new(CUSTOMER);
        let mut customer = Customer::from(&org1);

        let code_new = Characteristic {
            name : "code".into(),
            value: "ABC".into(),
            value_type: "String".into()
        };
        let code_new_clone = code_new.clone();
        let code_old = customer.get_characteristic("code");
        let code_replace = customer.replace_characteristic(code_new);
        let code_replaced = customer.get_characteristic("code");

        // code_old and code_replace should be the same
        assert_eq!(code_old.unwrap().value,code_replace.unwrap().value);
        // code_new and code_replaced should be the same
        assert_eq!(code_new_clone.value,code_replaced.unwrap().value);
    }

    #[test]
    fn test_customer_code_whitespace() {
        // use default() to avoid id generation via new()
        let mut cust1 = Customer::default();
        cust1.set_id(CUSTOMER_UID.to_string());
        cust1.set_name(CUSTOMER);

        // Create second customer using name with whitespace
        let mut cust2 = Customer::default();
        cust2.set_id(CUSTOMER_UID.to_string());
        cust2.set_name(CUSTOMER_BAD);
        
        // Generate customer codes
        cust1.generate_code(None);
        cust2.generate_code(None);

        let code1 = cust1.get_characteristic("code").unwrap();
        let code2 = cust2.get_characteristic("code").unwrap();

        // Customer codes should be the same, but the ID is different.
        assert_eq!(code1.value,code2.value);
    }

    #[test]
    fn test_customer_characteristic_new_missing() {
        // Test replacing a non-existing characteristic
        let characteristic = Characteristic::from(("weather","rainy"));

        let org1 = Organization::new(CUSTOMER);
        let mut customer = Customer::from(&org1);

        customer.replace_characteristic(characteristic);

        let test_char = customer.get_characteristic("weather");

        assert!(test_char.is_some());
    }

    #[test]
    fn test_customer_characteristic_default_missing() {
        // Test replacing a non-existing characteristic, on a default Customer (i.e. no Vec creatd)
        let characteristic = Characteristic::from(("weather","rainy"));

        let mut customer = Customer::default();

        customer.replace_characteristic(characteristic);

        let test_char = customer.get_characteristic("weather");

        assert!(test_char.is_some());
    }
}

