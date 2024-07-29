//! Resource Module

use serde::{Deserialize,Serialize};

use crate::common::related_party::RelatedParty;
use crate::common::attachment::AttachmentRefOrValue;
use super::MOD_PATH;
use super::characteristic::Characteristic;
use crate::{
    HasId,
    HasName,
    HasAttachment,
    HasRelatedParty,
    LIB_PATH
};
use tmflib_derive::{
    HasId,
    HasName,
    HasAttachment,
    HasRelatedParty,
};

const CLASS_PATH : &str = "resource";
const RESOURCE_VERS : &str = "1.0";

/// Resource Usage Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum ResourceUsageStateType {
    /// Idle
    #[default]
    Idle,
    /// Active
    Active,
    /// Busy
    Busy,
}

/// Adminsitrative configuration of resource
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum ResourceAdministrativeStateType {
    /// Resource has been locked
    Locked,
    /// Resource has been unlocked
    #[default]
    Unlocked,
    /// Resource has been shutdown
    Shutdown,
}

/// Operational Status of resource
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum ResourceOperationalStateType {
    /// Resource is enabled
    #[default]
    Enable,
    /// Resource is disabled
    Disable,
}

/// Resource Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum ResourceStatusType {
    /// Standby
    Standby,
    /// Resource is in alarm
    Alarm,
    /// Resource is available
    #[default]
    Available,
    /// Resource has been reserved
    Reserved,
    /// Unknown status
    Unknown,
    /// Resource has been suspended
    Suspended,
}

/// TMF Resource 
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, HasAttachment, HasRelatedParty, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    administrative_state: ResourceAdministrativeStateType,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<Vec<AttachmentRefOrValue>>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP Uri
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Resource Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    operational_state: ResourceOperationalStateType,
    resource_characteristic: Vec<Characteristic>,
    resource_status: ResourceStatusType,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
    usage_state: ResourceUsageStateType,
}

impl Resource {
    /// Create a new Resource Inventory record
    pub fn new(name : impl Into<String>) -> Resource {
        let mut resource = Resource::create();
        resource.name = Some(name.into());
        resource.resource_version = Some(RESOURCE_VERS.to_owned());
        resource    
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RESOURCE_NAME : &str = "ResourceName";
    #[test]
    fn test_resource_new() {
        let resource = Resource::new(RESOURCE_NAME);

        assert_eq!(resource.get_name().as_str(),RESOURCE_NAME);
        assert_eq!(resource.resource_version.is_some(),true);
        assert_eq!(resource.resource_version.unwrap(),RESOURCE_VERS);
    }
}