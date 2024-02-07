//! Organization Module

use serde::{Deserialize, Serialize};

use crate::common::related_party::RelatedParty;
use crate::{CreateTMF, HasId, HasName, TimePeriod};
use tmflib_derive::{HasId,HasName};

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

/// Reference to an organization
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationRef {
    /// Referenced Org Id
    pub id: String,
    /// Referenced Org Uri
    pub href: String,
    /// Referenced Org Name
    pub name : String,
}

impl From<Organization> for OrganizationRef {
    fn from(value: Organization) -> Self {
        OrganizationRef {
            id: value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
        }
    }
}

/// Organisation record (sic)
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    /// Unique id of this organization record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML reference to this organization record
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Flag this organisation as the head office
    #[serde(skip_serializing_if = "Option::is_none")]
    is_head_office: Option<bool>,
    /// Flag this organisation as a legal entity
    #[serde(skip_serializing_if = "Option::is_none")]
    is_legal_entity: Option<bool>,
    /// Name of this Organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Type of name of this Organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_type: Option<String>,
    /// Type of Organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_type: Option<String>,
    /// Trading Name of this Organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_name: Option<String>,
    /// Existence Period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists_during: Option<TimePeriod>,
    
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrganizationStateType>,
    
    /// Contact medium for organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_medium: Option<Vec<ContactMedium>>,
    /// Related Party
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    
}

impl Organization {
    /// Create a new organization record with a name
    pub fn new(name : impl Into<String>) -> Organization {
        let mut org = Organization::create();
        org.name = Some(name.into());
        org.related_party = Some(vec![]);
        org
    }
}

impl From<String> for Organization {
    fn from(value: String) -> Self {
        // Generate an Organization from a given string, treating String as name
        Organization::new(value)
    }
}