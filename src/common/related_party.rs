//! Related Party Module

use std::convert::From;
use serde::{Deserialize,Serialize};

use crate::tmf629::customer::Customer;
use crate::tmf632::individual::Individual;
use crate::tmf632::organization::Organization;

#[derive(Clone, Debug, Default, Deserialize, Serialize )]
pub struct RelatedParty {
    pub id: String,
    pub href: String,
    pub name: String,
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
    }
    #[test]
    fn test_related_party_from_customer_role() {
        let cust = Customer::new(String::from("ACustomer"));
        let party = RelatedParty::from(&cust);
        assert_eq!(party.role, String::from("customer"));
    }
}



