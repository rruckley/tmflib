//! Organization Module

use serde::{Deserialize, Serialize};

use crate::{CreateTMF, HasId};

use crate::LIB_PATH;
use super::MOD_PATH;
use crate::common::contact::ContactMedium;

const ORG_PATH : &str = "organization";

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Organization {
    contact_medium: Vec<ContactMedium>,
    pub id: Option<String>,
    pub href: Option<String>,
    pub name: String,
}

impl Organization {
    pub fn new(name : String) -> Organization {
        let mut org = Organization::default();
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
        let id = self.get_uuid();
        self.id = Some(id);    
    }
    fn get_href(&mut self) -> String {
        self.href.as_ref().unwrap().clone()
    }
    fn get_id(&mut self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
}