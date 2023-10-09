//! Related Party Module

use std::convert::From;
use serde::{Deserialize,Serialize};

use crate::tmf629::customer::Customer;

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
            role: "customer".to_string() 
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



