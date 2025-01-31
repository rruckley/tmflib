//! Check Service Qualification Module

use serde::{Deserialize,Serialize};

use crate::{
    common::related_party::RelatedParty, HasDescription, HasId, HasRelatedParty, Uri, LIB_PATH
};

use tmflib_derive::{
    HasId,
    HasDescription,
    HasRelatedParty,
};

const CLASS_PATH : &str = "checkServiceQualification";
use super::MOD_PATH;

/// Check Service Qualification
#[derive(Clone,Debug,Default,HasId,HasDescription,HasRelatedParty, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckServiceQualificaitonItem {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id : Option<String>,
    /// HTTP URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    // Linked Objects
    /// Related Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party : Option<Vec<RelatedParty>>,
}

/// Check Service Qualification 
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct CheckServiceQualification {
    /// Service Qualification Items
    pub service_qualification_item : Option<Vec<CheckServiceQualificaitonItem>>,
}