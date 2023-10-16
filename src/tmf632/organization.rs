//! Organization Module

use serde::{Deserialize, Serialize};

use crate::{CreateTMF, HasId};

use crate::LIB_PATH;
use super::MOD_PATH;
use crate::common::contact::ContactMedium;

const ORG_PATH : &str = "organization";

/// Organisation record (sic)
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    contact_medium: Vec<ContactMedium>,
    /// Unique id of this organization record
    pub id: Option<String>,
    /// HTML reference to this organization record
    pub href: Option<String>,
    /// Name of this Organization
    pub name: String,
}

impl Organization {
    /// Create a new organization record with a name
    pub fn new(name : String) -> Organization {
        let mut org = Organization::create();
        org.name = name;
        org
    }
}

impl CreateTMF<Organization> for Organization {}

impl HasId for Organization {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,ORG_PATH,self.get_id());
        self.href = Some(href);   
    }
    fn generate_id(&mut self) {
        let id = Organization::get_uuid();
        self.id = Some(id);
        self.generate_href();
    }
    fn get_href(&mut self) -> String {
        self.href.as_ref().unwrap().clone()
    }
    fn get_id(&mut self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
}

impl From<String> for Organization {
    fn from(value: String) -> Self {
        // Generate an Organization from a given string, treating String as name
        Organization::new(value)
    }
}