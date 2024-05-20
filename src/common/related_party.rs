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
#[cfg(feature = "tmf632-v4")]
use crate::tmf632::individual_v4::Individual;
#[cfg(feature = "tmf632-v5")]
use crate::tmf632::individual_v5::Individual;
#[cfg(feature = "tmf632-v4")]
use crate::tmf632::organization_v4::{Organization,OrganizationRef};
#[cfg(feature = "tmf632-v5")]
use crate::tmf632::organization_v5::{Organization,OrganizationRef};
use crate::tmf669::party_role::PartyRole;
use crate::{HasId,HasName,HasRefHRef,HasRefId,IsRef,Uri};

/// Reference to a Customer (TMF629) , Organisation or Individual (TMF632)
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize )]
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
  
    // META
    /// Base Type this type is derived from if creating sub-classes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    pub base_type : Option<String>,
    /// Schema Definition of the sub-class (if required)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    pub schema_location: Option<Uri>,
    /// Name for this Type when sub-classing
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type : Option<String>,
    /// What type is this reference referring to?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@referredType")]
    pub referred_type : Option<String>,
}

impl HasRefId for RelatedParty {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl HasRefHRef for RelatedParty {
    fn get_href(&self) -> String {
        self.href.clone()
    }
}

impl IsRef for RelatedParty {}

impl From<&Customer> for RelatedParty {
    fn from(cust: &Customer) -> Self {
        RelatedParty { 
            id: cust.id.as_ref().unwrap().clone(), 
            href: cust.href.as_ref().unwrap().clone(), 
            name: cust.name.clone(),
            role: Some(Customer::get_class()),
            base_type: Some(Customer::get_class()),
            referred_type: Some(Customer::get_class()),
            r#type: Some(Customer::get_class()),
            schema_location: None,
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
            referred_type: Some(Organization::get_class()),
            base_type: Some(Organization::get_class()),
            r#type : Some(Organization::get_class()),
            schema_location: None,
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
            referred_type: Some(Organization::get_class()),
            base_type: Some(Organization::get_class()),
            r#type : Some(Organization::get_class()),
            schema_location: None,
        }
    }
}

impl From<&Individual> for RelatedParty {
    fn from(value: &Individual) -> Self {
        RelatedParty { 
            id: value.id.as_ref().unwrap().clone(), 
            href: value.href.as_ref().unwrap().clone(), 
            name: value.full_name.clone(), 
            role: Some(Individual::get_class()),
            referred_type: Some(Individual::get_class()),
            base_type: Some(Individual::get_class()),
            r#type : Some(Individual::get_class()),
            schema_location: None,
        }
    }
}

/// See: <https://engage.tmforum.org/discussion/role-in-relatedparty?ReturnUrl=%2fcommunities%2fcommunity-home%2fdigestviewer%3fcommunitykey%3dd543b8ba-9d3a-4121-85ce-5b68e6c31ce5>
/// Create a [RelatedParty] reference from a reference to [crate::tmf669::party_role::PartyRole]
impl From<&PartyRole> for RelatedParty {
    fn from(value: &PartyRole) -> Self {
        RelatedParty { 
            id: value.id.as_ref().unwrap().clone(), 
            href: value.href.as_ref().unwrap().clone(), 
            name: None, 
            role: value.name.clone(),
            referred_type: Some(PartyRole::get_class()),
            base_type: Some(PartyRole::get_class()),
            r#type : Some(PartyRole::get_class()),
            schema_location: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tmf629::customer::Customer;
    #[cfg(feature = "tmf632-v4")]
    use crate::tmf632::organization_v4::Organization;
    #[cfg(feature = "tmf632-v5")]
    use crate::tmf632::organization_v5::Organization;
    use crate::{HasId,HasRefId,HasRefHRef,IsRef};
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
    #[test]
    fn test_related_party_from_customer_referred() {
        let org = Organization::new(String::from("ACustomer"));
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);

        assert_eq!(party.referred_type.unwrap(), Customer::get_class());
    }

    #[test]
    fn test_related_party_id() {
        let org = Organization::new(String::from("ACustomer"));
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);

        assert_eq!(cust.get_id(),party.get_id()); 
    }

    #[test]
    fn test_related_party_href() {
        let org = Organization::new(String::from("ACustomer"));
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);

        assert_eq!(cust.get_href(),party.get_href());
    }

    #[test]
    fn test_related_party_hydrate() {
        let org = Organization::new(String::from("ACustomer"));
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);

        let out = party.hydrate_ref(|h| {
            Some(h.clone())
        });

        assert_eq!(party.get_href(),out.unwrap());

    }
}



