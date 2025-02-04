//! Resource Function Module

use serde::{Deserialize, Serialize};

use crate::{HasId, HasName, LIB_PATH,HasDescription,HasRelatedParty,Uri,Priority,DateTime};
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

/// Administrative Status
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum ResourceAdministrativeStateType {
    /// Resource is Locked
    Locked,
    /// Resource is unlocked
    #[default]
    Unlocked,
    /// Resource is shutdown
    Shutdown,
}

/// Operational Status
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum ResourceOperationalStateType {
    /// Resource is enabled
    Enable,
    /// Resource is disabled
    #[default]
    Disable,
}

/// Usage Status
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum ResourceUsageStateType {
    Idle,
    Busy,
    #[default]
    Active,
}

/// Resource Function
#[derive(Clone,Debug,Default,HasId,HasName,HasDescription,HasRelatedParty,Deserialize,Serialize)]
pub struct ResourceFunction {
    pub administrative_state: Option<ResourceAdministrativeStateType>,
    /// Category of the concrete resource. e.g. Gold, Silver for MSISDN concrete resource.
    pub category : Option<String>,
    /// The date till the resource is operating.
    pub end_operating_date : Option<DateTime>,
    /// A type of the Resource Function as specified by the provider of the API.
    pub function_type : Option<String>,

    pub operational_state : Option<ResourceOperationalStateType>,
    pub priority : Option<Priority>,
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
    pub role : Option<String>,
    pub start_operating_date: Option<DateTime>,
    pub usage_state : Option<ResourceUsageStateType>,
    /// The value of the logical resource. e.g. '07439823498' for MSISDNs
    pub value : Option<String>,
    // Reference Modules
    /// Related Parties
    pub related_party : Option<Vec<RelatedParty>>,
}