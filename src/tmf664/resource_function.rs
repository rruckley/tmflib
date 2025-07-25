//! Resource Function Module

use serde::{Deserialize, Serialize};

use crate::{HasId, HasName, LIB_PATH,HasDescription,HasRelatedParty,HasNote,HasValidity,Uri,Priority,DateTime,TimePeriod};
use tmflib_derive::{HasId,HasName,HasDescription,HasRelatedParty,HasValidity,HasNote};
use crate::common::related_party::RelatedParty;
use crate::common::note::Note;
use crate::common::tmf_error::TMFError;

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
    /// Function is idle
    Idle,
    /// Function is busy
    Busy,
    /// Function is active
    #[default]
    Active,
}

/// Configuration Feature
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct Feature {
    // Referenced objects
    /// Relationships
    pub feature_relationship : Option<Vec<FeatureRelationship>>,
}

/// Feature Relationship
#[derive(Clone,Debug,Default,Deserialize,HasId,HasName,HasValidity,Serialize)]
pub struct FeatureRelationship {
    id : Option<String>,
    href: Option<Uri>,
    name : Option<String>,
    relationship_type : Option<String>,
    valid_for : Option<TimePeriod>,
}

/// # Resource Function
/// A ResourceFunction is a behaviour to transform inputs of any nature into outputs of any 
/// nature independently from the way it is provided.
#[derive(Clone,Debug,Default,HasId,HasName,HasDescription,HasRelatedParty,HasNote,Deserialize,Serialize)]
pub struct ResourceFunction {
    /// Administrative Status
    pub administrative_state: Option<ResourceAdministrativeStateType>,
    /// Category of the concrete resource. e.g. Gold, Silver for MSISDN concrete resource.
    pub category : Option<String>,
    /// The date till the resource is operating.
    pub end_operating_date : Option<DateTime>,
    /// A type of the Resource Function as specified by the provider of the API.
    pub function_type : Option<String>,
    /// Operational Status
    pub operational_state : Option<ResourceOperationalStateType>,
    /// Function Priority
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
    /// Function Role
    pub role : Option<String>,
    /// Operational Start Date
    pub start_operating_date: Option<DateTime>,
    /// Usage Status
    pub usage_state : Option<ResourceUsageStateType>,
    /// The value of the logical resource. e.g. '07439823498' for MSISDNs
    pub value : Option<String>,
    // Reference Modules
    /// Related Parties
    pub related_party : Option<Vec<RelatedParty>>,
    /// Activation Feature
    pub activation_feature : Option<Vec<Feature>>,
    /// Notes
    pub note : Option<Vec<Note>>,
}