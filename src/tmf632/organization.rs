//! Organization Module

use serde::{Deserialize, Serialize};

use crate::common::related_party::RelatedParty;
use crate::{CreateTMF, HasId, HasName};

use crate::LIB_PATH;
use super::MOD_PATH;
use crate::common::contact::ContactMedium;

const ORG_PATH : &str = "organization";

/// Organization Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrganizationStateType {
    /// Initialized
    #[default]
    Initialized,
    /// Validation complete
    Validated,
    /// No longer valid
    Closed,
}

/// Organisation record (sic)
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_medium: Option<Vec<ContactMedium>>,
    /// Unique id of this organization record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML reference to this organization record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Name of this Organization
    pub name: String,
    /// Related Party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Status
    pub status: Option<OrganizationStateType>,
}

impl Organization {
    /// Create a new organization record with a name
    pub fn new(name : String) -> Organization {
        let mut org = Organization::create();
        org.name = name;
        org.related_party = Some(vec![]);
        org
    }
}

impl HasName for Organization {
    fn get_name(&self) -> String {
        self.name.clone()
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
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
    fn get_class() -> String {
        ORG_PATH.to_owned()
    }
}

impl From<String> for Organization {
    fn from(value: String) -> Self {
        // Generate an Organization from a given string, treating String as name
        Organization::new(value)
    }
}