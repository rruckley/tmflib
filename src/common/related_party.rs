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
use crate::{HasId,HasName,Uri};

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

impl From<&Customer> for RelatedParty {
    fn from(cust: &Customer) -> Self {
        RelatedParty { 
            id: cust.get_id(), 
            href: cust.get_href(), 
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

impl From<&Organization> for RelatedParty {
    fn from(org: &Organization) -> Self {
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
    use crate::{tmf629::customer::Customer, tmf632::organization_v4::OrganizationRef};
    #[cfg(feature = "tmf632-v4")]
    use crate::tmf632::organization_v4::Organization;
    #[cfg(feature = "tmf632-v5")]
    use crate::tmf632::organization_v5::Organization;
    use crate::tmf669::party_role::PartyRole;
    use crate::{HasId, HasName};
    use super::RelatedParty;

    const ORG_NAME : &str = "An Organisation";
    const ROLE_NAME : &str = "A Role";

    #[test]
    fn test_related_party_from_customer_id() {
        let org = Organization::new(ORG_NAME);
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);
        assert_eq!(cust.id.unwrap(), party.id);
    }
    #[test]
    fn test_related_party_from_customer_href() {
        let org = Organization::new(ORG_NAME);
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);
        assert_eq!(cust.href.unwrap(), party.href);
    }
    #[test]
    fn test_related_party_from_customer_name() {
        let org = Organization::new(ORG_NAME);
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);
        assert_eq!(cust.name, party.name);
    }
    #[test]
    fn test_related_party_from_customer_role() {
        let org = Organization::new(ORG_NAME);
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);
        assert_eq!(party.role.unwrap(), Customer::get_class());
        
    }
    #[test]
    fn test_related_party_from_customer_referred() {
        let org = Organization::new(ORG_NAME);
        let cust = Customer::new(org);
        let party = RelatedParty::from(&cust);

        assert_eq!(party.referred_type.unwrap(), Customer::get_class());
    }

    #[test]
    fn test_related_party_from_organization() {
        let org = Organization::new(ORG_NAME);

        let party = RelatedParty::from(&org);

        assert_eq!(org.name,party.name);
        assert_eq!(org.id.unwrap(),party.id);
        assert_eq!(org.href.unwrap(),party.href);
    }

    #[test]
    fn test_related_party_from_organization_ref() {
        let org = Organization::new(ORG_NAME);
        let org_ref = OrganizationRef::from(org);

        let party = RelatedParty::from(org_ref.clone());

        assert_eq!(org_ref.name,party.name.unwrap());
        assert_eq!(org_ref.id,party.id);
        assert_eq!(org_ref.href,party.href);       
    }

    #[test]
    fn test_relatedparty_from_partyrole() {
        let party = Organization::new(ORG_NAME);
        let role = PartyRole::new(ROLE_NAME,RelatedParty::from(party.clone()));

        let new_party = RelatedParty::from(&role);

        assert_eq!(new_party.id,role.get_id());
        assert_eq!(new_party.role.is_some(),true);
        assert_eq!(new_party.role.unwrap(),role.get_name());
        assert_eq!(new_party.referred_type.is_some(),true);
        assert_eq!(new_party.referred_type.unwrap(),PartyRole::get_class());
    }
}



