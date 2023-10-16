//! Customer Module
//!
use serde::{Deserialize, Serialize};
use sha256::digest;
use uuid::Uuid;

use crate::CreateTMF;
use crate::tmf632::organization::Organization;

use super::characteristic::Characteristic;
use crate::common::contact::ContactMedium;
use crate::common::related_party::RelatedParty;
use super::HasId;
use super::LIB_PATH;
use super::MOD_PATH;

const CUST_PATH : &str = "customer";
const CUST_ID_SIZE : usize = 5;
/// Default customer status
pub const CUST_STATUS : &str = "New";

/// Customer object
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    /// Html Reference to this object
    pub href: Option<String>,
    /// Unique Id
    pub id: Option<String>,
    /// Name of this 
    pub name: String,
    /// Customer status
    pub status: Option<String>,
    /// Reason for current status
    pub status_reason: Option<String>,
    /// Validity of this record
    pub valid_for: Option<String>,
    contact_medium: Option<Vec<ContactMedium>>,
    characteristic: Option<Vec<Characteristic>>,
    related_party: Vec<RelatedParty>,
    engaged_party: Option<RelatedParty>,
}

impl CreateTMF<Customer> for Customer {}

impl Customer {
    /// Create new customer object
    pub fn new(org: Organization) -> Customer {
        let mut cust = Customer::create();
        cust.name = org.name.clone();
        // Not sure on including the name here but the id is only generated on create(), so a name change would
        // not impact the generated code. Ideally as we're throwing away a log of the resulting hash to get the
        // code, it might help avoid collisions if we add some more entropy?
        let hash_input = format!("{}:{}",cust.get_id(),cust.name);
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
        cust.engaged_party = Some(RelatedParty::from(org));
        
        cust.status = Some(CUST_STATUS.to_string());
        cust.contact_medium = Some(vec![]);
        cust.characteristic= Some(vec![code, hash]);
        cust
    }

    /// Geneate a unique customer code via cryptographic functions
    pub fn generate_code(&mut self) {
        // Generate a new code based on name

        // Generate Id if none exists
        if self.id.is_none() {
            self.generate_id();
        };
        let hash_input = format!("{}:{}", self.id.as_ref().unwrap(), self.name);
        let sha = digest(hash_input);
        let sha_slice = sha.as_str()[..4].to_string().to_ascii_uppercase();
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

        self.characteristic.as_mut().unwrap().push(code);
        self.characteristic.as_mut().unwrap().push(hash);
    }

    /// Set the name of the customer
    pub fn name(&mut self, name : String) {
        self.name = name.clone();
    }
}

impl HasId for Customer {
    fn get_id(&mut self) -> String {
        match &self.id {
            None => {
                self.generate_id();
                self.id.as_ref().unwrap().clone()
            }
            Some(id) => id.to_string(),
        }
    }

    fn get_href(&mut self) -> String {
  
        if self.href.is_none() {
            self.generate_href();
        }
        self.href.as_ref().unwrap().clone()
    }

    fn generate_id(&mut self) {
        let id = Uuid::new_v4().to_string();
        self.id = Some(id);
        // New id requires new href
        self.generate_href();
    }

    fn generate_href(&mut self) {
        match &self.id {
            Some(_) => {
                let href = format!(
                    "/{}/{}/{}/{}",
                    LIB_PATH,
                    MOD_PATH,
                    CUST_PATH,
                    self.id.as_ref().unwrap()
                );
                self.href = Some(href);
            }
            None => self.generate_id(),
        }
    }
}

impl From<&Organization> for Customer {
    fn from(value: &Organization) -> Self {
        let mut customer = Customer::new(value.to_owned());
        customer.generate_code();
        customer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_customer_new_name() {
        let org = Organization::new(String::from("ACustomer"));
        let customer = Customer::new(org);

        assert_eq!(customer.name, String::from("ACustomer"));
        assert_eq!(customer.id.is_some(),true);
        assert_eq!(customer.href.is_some(),true);
    }
}
