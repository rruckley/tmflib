//! Related Party Module
//! # Description
//! This object provides a reference object to any of the party objects, specifically:
//! - [`Customer`]
//! - [`PartyRole`]
//! - [`Individual`]
//! - [`Organization`]

use std::convert::From;
use serde::{Deserialize,Serialize};

use crate::tmf629::customer::Customer;
use crate::tmf632::individual::Individual;
use crate::tmf632::organization::{Organization,OrganizationRef};
use crate::tmf669::party_role::PartyRole;
use crate::{HasId,HasName};

/// Reference to a Customer (TMF629) , Organisation or Individual (TMF632)
#[derive(Clone, Debug, Default, Deserialize, Serialize )]
#[serde(rename_all = "camelCase")]
pub struct RelatedParty {
    /// Unique Id of the referenced party
    pub id: String,
    /// HTML reference of the related party
    pub href: String,
    /// Name of the referenced party / customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name referenced role 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

impl From<&Customer> for RelatedParty {
    fn from(cust: &Customer) -> Self {
        RelatedParty { 
            id: cust.id.as_ref().unwrap().clone(), 
            href: cust.href.as_ref().unwrap().clone(), 
            name: cust.name.clone(),
            role: Some(Customer::get_class()),
        }    
    }
}

impl From<Organization> for RelatedParty {
    fn from(org : Organization) -> Self {
        RelatedParty { 
            id: org.get_id(), 
            href: org.get_href(), 
            name: Some(org.get_name()), 
            role: Some(Organization::get_class()),
        }
    }
}

impl From<OrganizationRef> for RelatedParty {
    fn from(value: OrganizationRef) -> Self {
        RelatedParty {
            id: value.id.clone(), 
            href: value.href.clone(), 
            name: Some(value.name.clone()), 
            role: Some(Organization::get_class()),    
        }
    }
}

impl From<&Individual> for RelatedParty {
    fn from(value: &Individual) -> Self {
        RelatedParty { 
            id: value.id.as_ref().unwrap().clone(), 
            href: value.href.as_ref().unwrap().clone(), 
            name: Some(value.full_name.clone()), 
            role: Some(Individual::get_class()),
        }
    }
}

/// See: <https://engage.tmforum.org/discussion/role-in-relatedparty?ReturnUrl=%2fcommunities%2fcommunity-home%2fdigestviewer%3fcommunitykey%3dd543b8ba-9d3a-4121-85ce-5b68e6c31ce5>
/// 
impl From<&PartyRole> for RelatedParty {
    fn from(value: &PartyRole) -> Self {
        RelatedParty { 
            id: value.id.as_ref().unwrap().clone(), 
            href: value.href.as_ref().unwrap().clone(), 
            name: None, 
            role: value.name.clone()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tmf629::customer::Customer;
    use crate::tmf632::organization::Organization;
    use crate::HasId;
    use super::RelatedParty;
    #[test]
    fn test_related_party_from_customer_id() {
        let org = Organization::new(String::from("ACustomer"));
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);
        assert_eq!(cust.id.unwrap(), party.id);
    }
    #[test]
    fn test_related_party_from_customer_href() {
        let org = Organization::new(String::from("ACustomer"));
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);
        assert_eq!(cust.href.unwrap(), party.href);
    }
    #[test]
    fn test_related_party_from_customer_name() {
        let org = Organization::new(String::from("ACustomer"));
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);
        assert_eq!(cust.name, party.name);
    }
    #[test]
    fn test_related_party_from_customer_role() {
        let org = Organization::new(String::from("ACustomer"));
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);
        assert_eq!(party.role.unwrap(), Customer::get_class());
    }
}



