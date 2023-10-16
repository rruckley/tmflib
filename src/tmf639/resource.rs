//! Resource Module

use serde::{Deserialize,Serialize};
use uuid::Uuid;

use crate::common::related_party::RelatedParty;
use super::MOD_PATH;
use crate::{HasId, CreateTMF, LIB_PATH};

const RESOURCE_PATH : &str  = "resource";

/// TMF Resource 
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    id: Option<String>,
    href: Option<String>,
    name: String,
    related_party: Vec<RelatedParty>,
}

impl CreateTMF<Resource> for Resource {}

impl Resource {
    /// Create a new Resource Inventory record
    pub fn new(name : &str) -> Resource {
        let mut resource = Resource::create();
        resource.name = name.to_owned();
        resource    
    }

    /// Add related party to this resource
    pub fn add_party(&mut self, party: RelatedParty) {
        self.related_party.push(party);
    }
}

impl HasId for Resource {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,RESOURCE_PATH,self.get_id());
        self.href = Some(href);       
    }
    fn generate_id(&mut self) {
        let id = Uuid::new_v4().simple().to_string();
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