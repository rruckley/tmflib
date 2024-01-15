//! Organization Module

use serde::{Deserialize, Serialize};

use crate::common::related_party::RelatedParty;
use crate::{CreateTMF, HasId, HasName};
use tmflib_derive::HasId;

use crate::LIB_PATH;
use super::MOD_PATH;
use crate::common::contact::ContactMedium;

const CLASS_PATH : &str = "organization";

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
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
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
    pub fn new(name : impl Into<String>) -> Organization {
        let mut org = Organization::create();
        org.name = name.into();
        org.related_party = Some(vec![]);
        org
    }
}

impl HasName for Organization {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl From<String> for Organization {
    fn from(value: String) -> Self {
        // Generate an Organization from a given string, treating String as name
        Organization::new(value)
    }
}