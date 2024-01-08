//! Individual Module

use serde::{Deserialize, Serialize};

use crate::{HasId,HasName,CreateTMF};
use crate::LIB_PATH;
use super::MOD_PATH;
use crate::common::related_party::RelatedParty;
use crate::common::contact::ContactMedium;

const IND_PATH : &str = "individual";

/// An individual
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Individual {
    /// Methods for contacting this individual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<ContactMedium>>,
    /// Unique id for this individual
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML reference for this individual object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Full name of the individual
    pub full_name: String,
    /// Parties related to this individual, e.g. company / organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

impl CreateTMF<Individual> for Individual {}

impl Individual {
    /// Create a new instance of indiviudal object
    pub fn new(name : impl Into<String>) -> Individual {
        let mut ind = Individual::create();
        ind.full_name = name.into();
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

impl HasId for Individual {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,IND_PATH,self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = Individual::get_uuid();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()   
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone() 
    }
    fn get_class() -> String {
        IND_PATH.to_owned()
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