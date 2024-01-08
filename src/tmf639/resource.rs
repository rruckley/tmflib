//! Resource Module

use serde::{Deserialize,Serialize};
use uuid::Uuid;

use crate::common::related_party::RelatedParty;
use crate::common::attachment::AttachmentRefOrValue;
use super::MOD_PATH;
use super::characteristic::Characteristic;
use crate::{HasId, CreateTMF, LIB_PATH};

const RESOURCE_PATH : &str = "resource";
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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    administrative_state: ResourceAdministrativeStateType,
    attachment: Vec<AttachmentRefOrValue>,
    id: Option<String>,
    href: Option<String>,
    name: String,
    operational_state: ResourceOperationalStateType,
    resource_characteristic: Vec<Characteristic>,
    resource_status: ResourceStatusType,
    resource_version: Option<String>,
    related_party: Vec<RelatedParty>,
    usage_state: ResourceUsageStateType,
}

impl CreateTMF<Resource> for Resource {}

impl Resource {
    /// Create a new Resource Inventory record
    pub fn new(name : impl Into<String>) -> Resource {
        let mut resource = Resource::create();
        resource.name = name.into();
        resource.resource_version = Some(RESOURCE_VERS.to_owned());
        resource    
    }

    /// Add related party to this resource
    pub fn add_party(&mut self, party: RelatedParty) {
        self.related_party.push(party);
    }
}

impl HasId for Resource {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,RESOURCE_PATH,self.get_id());
        self.href = Some(href);       
    }
    fn generate_id(&mut self) {
        let id = Uuid::new_v4().simple().to_string();
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
        RESOURCE_PATH.to_owned()
    }
}