//! Individual Module

use serde::{Deserialize, Serialize};

use crate::{HasId,CreateTMF};
use crate::LIB_PATH;
use super::MOD_PATH;
use crate::common::related_party::RelatedParty;
use crate::common::contact::ContactMedium;

const IND_PATH : &str = "individual";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Individual {
    pub contact_medium: Vec<ContactMedium>,
    pub id: Option<String>,
    pub href: Option<String>,
    pub full_name: String,
    pub related_party: Vec<RelatedParty>,
}

impl CreateTMF<Individual> for Individual {}

impl Individual {
    pub fn new(name : String) -> Individual {
        let mut ind = Individual::create();
        ind.full_name = name;
        ind.related_party = vec![];
        ind
    }

    /// Add a related party to the individual
    pub fn add_party(&mut self, party : RelatedParty) {
        self.related_party.push(party);
    }

    /// Add a contact medium to the individual
    pub fn add_contact(&mut self, medium : ContactMedium) {
        self.contact_medium.push(medium);
    }
}

impl HasId for Individual {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,IND_PATH,self.get_id());
        self.href = Some(href);    
    }
    fn generate_id(&mut self) {
        let id = self.get_uuid();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_href(&mut self) -> String {
        self.href.as_ref().unwrap().clone()   
    }
    fn get_id(&mut self) -> String {
        if self.id.is_none() {
            self.generate_id();
        }
        self.id.as_ref().unwrap().clone() 
    }
}

#[cfg(test)]
mod test {
    use super::Individual;
    #[test]
    fn test_individual_create_id() {
        let ind = Individual::new(String::from("APerson"));

        assert_eq!(ind.id.is_some(),true);
    }

    #[test]
    fn test_individual_create_href() {
        let ind = Individual::new(String::from("APerson"));

        assert_eq!(ind.href.is_some(),true);
    }
}