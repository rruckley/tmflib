//! Party Role Module

use serde::{Deserialize, Serialize};

use crate::{HasId,CreateTMF,LIB_PATH, common::related_party::RelatedParty};

use super::MOD_PATH;

const ROLE_PATH : &str = "role";

/// Party Role
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PartyRole {
    /// Id of the Party Role
   pub id: Option<String>,
   /// HTML link
   pub href: Option<String>,
   /// Name of Role
   pub name: String,
   /// Entity that is associated with this role
   engaged_party: Option<RelatedParty>,
   /// Other related parties
   related_party: Vec<RelatedParty>,
}

impl PartyRole {
    /// Create new PartyRole with given name
    pub fn new(name : &str) -> PartyRole {
        let mut role = PartyRole::create();
        role.name = name.to_owned();
        role
    }
}

impl HasId for PartyRole {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,ROLE_PATH,self.get_id());
        self.href = Some(href);
    }
    fn generate_id(&mut self) {
        let id = PartyRole::get_uuid();
        self.id = Some(id);
        self.generate_href();    
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()       
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()
    }
}

impl CreateTMF<PartyRole> for PartyRole {}