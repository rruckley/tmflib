//! Resource Function Module

use serde::{Deserialize, Serialize};

use crate::{HasId, HasName, LIB_PATH,HasDescription,HasRelatedParty,Uri};
use tmflib_derive::{HasId,HasName,HasDescription,HasRelatedParty};
use crate::common::related_party::RelatedParty;

use super::MOD_PATH;

const CLASS_PATH : &str = "resourceFunction";

/// Reference to Resource Function
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct ResourceFunctionRef {
    /// Referenced Uri
    pub href : Uri,
    /// Referenced Id
    pub id : String,
    /// Referenced Name
    pub name: String,
    /// Referenced Version
    pub version : String,
}

impl From<ResourceFunction> for ResourceFunctionRef {
    fn from(value: ResourceFunction) -> Self {
        ResourceFunctionRef {
            href : value.get_href(),
            id : value.get_id(),
            name : value.get_name(),
            version: value.resource_version.unwrap_or("1.0".to_string()),
        }
    }
}

/// Resource Function
#[derive(Clone,Debug,Default,HasId,HasName,HasDescription,HasRelatedParty,Deserialize,Serialize)]
pub struct ResourceFunction {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP Uri
    pub href : Option<Uri>,
    /// Name
    pub name : Option<String>,
    /// Description
    pub description : Option<String>,
    /// Version of resource
    pub resource_version : Option<String>,
    // Reference Modules
    /// Related Parties
    pub related_party : Option<Vec<RelatedParty>>,
}