//! Resource Module

use serde::{Deserialize, Serialize};

use super::characteristic::Characteristic;
use super::MOD_PATH;
use crate::common::attachment::AttachmentRefOrValue;
use crate::common::related_party::RelatedParty;
use crate::common::tmf_error::TMFError;
use crate::{HasAttachment, HasId, HasName, HasRelatedParty, LIB_PATH};
use tmflib_derive::{HasAttachment, HasId, HasName, HasRelatedParty};

const CLASS_PATH: &str = "resource";
const RESOURCE_VERS: &str = "1.0";

/// Resource Usage Status
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ResourceOperationalStateType {
    /// Resource is enabled
    #[default]
    Enable,
    /// Resource is disabled
    Disable,
}

/// Resource Status
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(
    Clone, Debug, Default, Deserialize, HasId, HasName, HasAttachment, HasRelatedParty, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// Administrative Status
    pub administrative_state: ResourceAdministrativeStateType,
    /// Attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP Uri
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Resource Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Operational Status
    pub operational_state: ResourceOperationalStateType,
    /// Characteristics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_characteristic: Option<Vec<Characteristic>>,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<ResourceStatusType>,
    /// Version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
    /// Related Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// Usage Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_state: Option<ResourceUsageStateType>,
}

impl Resource {
    /// Create a new Resource Inventory record
    pub fn new(name: impl Into<String>) -> Resource {
        let mut resource = Resource::create();
        resource.name = Some(name.into());
        resource.resource_version = Some(RESOURCE_VERS.to_owned());
        resource
    }
}

#[cfg(test)]
mod test {
    use crate::common::related_party::RelatedParty;
    #[cfg(feature = "build-V4")]
    use crate::tmf632::individual_v4::Individual;
    #[cfg(feature = "build-V5")]
    use crate::tmf632::individual_v5::Individual;

    use super::*;

    const RESOURCE_NAME: &str = "ResourceName";

    const RESOUCEUSAGESTATE_JSON: &str = "\"Idle\"";
    const RESOUCEADMINADMINSTATE_JSON: &str = "\"Unlocked\"";
    const RESOUCEOPSSTATE_JSON: &str = "\"Enable\"";
    const RESOUCESTATE_JSON: &str = "\"Available\"";
    const RESOURCE_JSON: &str = "{
        \"administrativeState\" : \"Unlocked\",
        \"name\" : \"ResourceName\",
        \"operationalState\" : \"Enable\",
        \"resourceCharacteristic\" : [],
        \"resourceStatus\" : \"Available\",
        \"resourceVersion\" : \"1.0\",
        \"usageState\" : \"Idle\"
    }";

    const INDIVIDUAL_NAME: &str = "An Individual";

    #[test]
    fn test_resource_new() {
        let resource = Resource::new(RESOURCE_NAME);

        assert_eq!(resource.get_name().as_str(), RESOURCE_NAME);
        assert_eq!(resource.resource_version.is_some(), true);
        assert_eq!(resource.resource_version.unwrap(), RESOURCE_VERS);
    }

    #[test]
    fn test_resourceusagestate_deserialize() {
        let resourceusagestate: ResourceUsageStateType =
            serde_json::from_str(RESOUCEUSAGESTATE_JSON).unwrap();

        assert_eq!(resourceusagestate, ResourceUsageStateType::Idle);
    }

    #[test]
    fn test_resourceadminstate_deserialize() {
        let resourceadminstate: ResourceAdministrativeStateType =
            serde_json::from_str(RESOUCEADMINADMINSTATE_JSON).unwrap();

        assert_eq!(
            resourceadminstate,
            ResourceAdministrativeStateType::Unlocked
        );
    }

    #[test]
    fn test_resourceopsstate_deserialize() {
        let resourceopsstate: ResourceOperationalStateType =
            serde_json::from_str(RESOUCEOPSSTATE_JSON).unwrap();

        assert_eq!(resourceopsstate, ResourceOperationalStateType::Enable);
    }

    #[test]
    fn test_resourcestate_deseralize() {
        let resourcestate: ResourceStatusType = serde_json::from_str(RESOUCESTATE_JSON).unwrap();

        assert_eq!(resourcestate, ResourceStatusType::Available);
    }

    #[test]
    fn test_resource_deserialize() {
        let resource: Resource = serde_json::from_str(RESOURCE_JSON).unwrap();

        assert_eq!(resource.get_name().as_str(), "ResourceName");
        assert_eq!(resource.resource_version.is_some(), true);
        assert_eq!(resource.resource_version.unwrap().as_str(), "1.0");
        assert_eq!(
            resource.administrative_state,
            ResourceAdministrativeStateType::Unlocked
        );
        assert_eq!(
            resource.operational_state,
            ResourceOperationalStateType::Enable
        );
        assert_eq!(resource.usage_state, Some(ResourceUsageStateType::Idle));
    }

    #[test]
    fn test_resource_hasattachment() {
        let mut resource = Resource::new(RESOURCE_NAME);

        let attach = AttachmentRefOrValue::new();

        resource.add(&attach);

        assert_eq!(resource.attachment.is_some(), true);
        assert_eq!(resource.attachment.unwrap().len(), 1);
    }

    #[test]
    fn test_resource_hasrelatedparty() {
        let party = Individual::new(INDIVIDUAL_NAME);
        let mut resource = Resource::new(RESOURCE_NAME);
        resource.add_party(RelatedParty::from(&party));

        assert_eq!(resource.related_party.is_some(), true);
        assert_eq!(resource.related_party.unwrap().len(), 1);
    }
}
