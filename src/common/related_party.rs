//! Related Party Module

use std::convert::From;
use serde::{Deserialize,Serialize};

use crate::tmf629::customer::Customer;
use crate::tmf632::individual::Individual;
use crate::tmf632::organization::Organization;

/// Reference to a Customer (TMF629) , Organisation or Individual (TMF632)
#[derive(Clone, Debug, Default, Deserialize, Serialize )]
#[serde(rename_all = "camelCase")]
pub struct RelatedParty {
    /// Unique Id of the referenced party
    pub id: String,
    /// HTML reference of the related party
    pub href: String,
    /// Name of the related party
    pub name: String,
    /// Role of the relationship, e.g. Parent/Child
    pub role: String,
}

impl From<&Customer> for RelatedParty {
    fn from(cust: &Customer) -> Self {
        RelatedParty { 
            id: cust.id.as_ref().unwrap().clone(), 
            href: cust.href.as_ref().unwrap().clone(), 
            name: cust.name.clone(), 
            role: "Customer".to_string() 
        }    
    }
}

impl From<Organization> for RelatedParty {
    fn from(org : Organization) -> Self {
        RelatedParty { 
            id: org.id.as_ref().unwrap().clone(), 
            href: org.href.as_ref().unwrap().clone(), 
            name: org.name.clone(), 
            role: String::from("Organization"),
        }
    }
}

impl From<&Individual> for RelatedParty {
    fn from(value: &Individual) -> Self {
        RelatedParty { 
            id: value.id.as_ref().unwrap().clone(), 
            href: value.href.as_ref().unwrap().clone(), 
            name: value.full_name.clone(), 
            role: "Individual".to_string() 
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tmf629::customer::Customer;
    use super::RelatedParty;
    #[test]
    fn test_related_party_from_customer_id() {
        let cust = Customer::new(String::from("ACustomer"));
        let party = RelatedParty::from(&cust);
        assert_eq!(cust.id.unwrap(), party.id);
    }
    #[test]
    fn test_related_party_from_customer_href() {
        let cust = Customer::new(String::from("ACustomer"));
        let party = RelatedParty::from(&cust);
        assert_eq!(cust.href.unwrap(), party.href);
    }
    #[test]
    fn test_related_party_from_customer_name() {
        let cust = Customer::new(String::from("ACustomer"));
        let party = RelatedParty::from(&cust);
        assert_eq!(cust.name, party.name);
        assert_eq!(party.name, String::from("ACustomer"));
    }
    #[test]
    fn test_related_party_from_customer_role() {
        let cust = Customer::new(String::from("ACustomer"));
        let party = RelatedParty::from(&cust);
        assert_eq!(party.role, String::from("Customer"));
    }
}



