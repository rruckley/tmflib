//! Organization Module

use serde::{Deserialize, Serialize};
use chrono::{DateTime,Utc};
use uuid::Uuid;
use sha256::digest;
use hex::decode;
use base32::encode;

use crate::{
    CreateTMF, 
    HasId, 
    HasName, 
    TimePeriod,
    TMFEvent,
    LIB_PATH,
};
use tmflib_derive::{HasId,HasName};


use crate::common::{
    contact::ContactMedium,
    event::{Event,EventPayload},
    related_party::RelatedParty,
};

use super::{
    MOD_PATH,
    Characteristic,
};

const CLASS_PATH : &str = "organization";
const CODE_LENGTH : usize = 6;
const CODE_PREFIX : &str = "O-";

/// Organization Status
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    
    /// Party Characteristics
    #[serde(skip_serializing_if = "Option::is_none")]
    party_characteristic: Option<Vec<Characteristic>>,
}

impl Organization {
    /// Create a new organization record with a name
    pub fn new(name : impl Into<String>) -> Organization {
        let mut org = Organization::create();

        // Ensure name has been trimmed before settings
        let name : String = name.into();
        let name = name.trim();
        
        org.name = Some(name.to_string());
        org.status = Some(OrganizationStateType::default());
        org.related_party = Some(vec![]);
        org.generate_code(None);
        org
    }

        /// Generate a new site code based on available fields
        pub fn generate_code(&mut self, offset : Option<u32>) {
            let hash_input = format!("{}:{}:{}",self.get_id(),self.get_name(),offset.unwrap_or_default());
            let sha = digest(hash_input);
            let hex = decode(sha);
            let base32 = encode(base32::Alphabet::RFC4648 { padding: false }, hex.unwrap().as_ref());
            let sha_slice = base32.as_str()[..CODE_LENGTH].to_string().to_ascii_uppercase();
            let characteristic = Characteristic {
                name : String::from("code"),
                name_type : String::from("String"),
                value : format!("{}{}",CODE_PREFIX,sha_slice),
                ..Default::default()
            };
            self.replace_characteristic(characteristic);
        }
    
        /// Replace a characteristic returning the old value if found
        pub fn replace_characteristic(&mut self, characteristic : Characteristic) -> Option<Characteristic> {
            match self.party_characteristic.as_mut() {
                Some(c) => {
                    // Characteristic array exist
                    let pos = c.iter().position(|c| c.name == characteristic.name);
                    match pos {
                        Some(u) => {
                            // Clone old value for return
                            let old = c[u].clone();
                            // Replace
                            c[u] = characteristic;
                            Some(old)
                        },
                        None => {
                            // This means the characteristic could not be found, instead we insert it
                            // Additional we return None to indicate that no old value was found
                            c.push(characteristic);
                            None
                        },
                    }
                }
                None => {
                    // Characteristic Vec was not created yet, create it now.
                    self.party_characteristic = Some(vec![characteristic]);
                    // Return None to show no previous value existed.
                    None
                },
            }
        }
}

impl From<String> for Organization {
    fn from(value: String) -> Self {
        // Generate an Organization from a given string, treating String as name
        Organization::new(value)
    }
}

/// Organization Event Types
#[derive(Clone,Debug,Deserialize,Serialize)]
pub enum OrganizationEventType {
    /// Organization Created
    OrganizationCreateEvent,
    /// Organization Attribute Change
    OrganizationAttributeValueChangeEvent,
    /// Organization State Change
    OrganizationStateChangeEvent,
    /// Organization Deleted
    OrganizationDeleteEvent,
}

/// Organization Event
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct OrganizationEvent {
    /// Organization impacted by event
    pub organization : Organization,
}

impl TMFEvent<OrganizationEvent> for Organization {
    fn event(&self) -> OrganizationEvent {
        OrganizationEvent {
            organization : self.clone(),
        }    
    }    
}

impl EventPayload<OrganizationEvent> for Organization {
    type Subject = Organization;
    type EventType = OrganizationEventType;

    fn to_event(&self,event_type : Self::EventType) -> Event<OrganizationEvent,Self::EventType> {
        let now = Utc::now();
        let desc = format!("{:?} for {} [{}]",event_type,self.get_name(),self.get_id());
        //let event_time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        let event_time = DateTime::from_timestamp(now.timestamp(),0).unwrap();
        Event {
            correlation_id : None,
            description: Some(desc),
            domain: Some(Organization::get_class()),
            event_id: Uuid::new_v4().to_string(),
            field_path: None,
            href: Some(self.get_href()),
            id: Some(self.get_id()),
            title: Some(self.get_name()),
            event_time: event_time.to_string(),
            priority: None,
            time_occurred: Some(event_time.to_string()),
            event_type,
            event: self.event(),
        } 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_org_new_name() {
        let org = Organization::new("AnOrganization");

        assert_eq!(org.name,Some("AnOrganization".into()));
    }

    #[test]
    fn test_org_new_state() {
        let org = Organization::new("AnOrganization");

        assert_eq!(org.status,Some(OrganizationStateType::default()));
    }
}