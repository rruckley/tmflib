//! Individual Module

use serde::{Deserialize, Serialize};

use crate::{HasId,HasName,CreateTMF};
use tmflib_derive::HasId;
use crate::LIB_PATH;
use super::MOD_PATH;
use crate::common::related_party::RelatedParty;
use crate::common::contact::ContactMedium;

const CLASS_PATH : &str = "individual";

/// An individual
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Individual {
    /// Unique id for this individual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML reference for this individual object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aristocratic_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    death_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family_name_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formatted_name: Option<String>,
    /// Full name of the individual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Methods for contacting this individual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<ContactMedium>>,
  
    /// Parties related to this individual, e.g. company / organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

impl Individual {
    /// Create a new instance of indiviudal object
    pub fn new(name : impl Into<String>) -> Individual {
        let mut ind = Individual::create();
        // Try to split name into two parts
        // If it splits, take 1st as given name, second as family name
        let name = name.to_string();
        //let (given,family) = name.as_ref().split
        ind.full_name = Some(name.into());
        // Need this as default would be None
        ind.related_party = Some(vec![]);
        ind.contact_medium = Some(vec![]);
        ind
    }

    /// Convenience function to add an email contact medium
    /// # Example
    /// ```
    /// use tmflib::tmf632::individual::Individual;
    /// 
    /// let individual = Individual::new("John Smith")
    ///     .email("john.smith@example.com");
    /// ```
    pub fn email(mut self, email : &str) -> Individual {
        let medium = ContactMedium::email(email);
        self.add_contact(medium);
        self
    }

    /// Convenience funciton to add a mobile number contact medium
    /// # Example
    /// ```
    /// use tmflib::tmf632::individual::Individual;
    /// 
    /// let individual = Individual::new("John Smith")
    ///     .mobile("0411 111 111");
    /// ```
    pub fn mobile(mut self, mobile: &str) -> Individual {
        let medium = ContactMedium::mobile(mobile);
        self.add_contact(medium);
        self
    }

    /// Add a related party to the individual
    pub fn add_party(&mut self, party : RelatedParty) {
        self.related_party.as_mut().unwrap().push(party);
    }

    /// Add a contact medium to the individual
    pub fn add_contact(&mut self, medium : ContactMedium) {
        self.contact_medium.as_mut().unwrap().push(medium);
    }
}

impl HasName for Individual {
    fn get_name(&self) -> String {
        self.full_name.clone()
    }
}

#[cfg(test)]
mod test {
    use super::Individual;
    #[test]
    fn test_individual_create_id() {
        let ind = Individual::new("APerson");

        assert_eq!(ind.id.is_some(),true);
    }

    #[test]
    fn test_individual_create_href() {
        let ind = Individual::new("APerson");

        assert_eq!(ind.href.is_some(),true);
    }
}