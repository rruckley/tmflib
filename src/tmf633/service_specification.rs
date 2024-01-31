//! Service Specification Module

use serde::{Deserialize,Serialize};

use crate::{HasId,HasName,HasLastUpdate,CreateTMF,CreateTMFWithTime, TimePeriod};
use crate::common::related_party::RelatedParty;
use tmflib_derive::{HasId,HasName,HasLastUpdate};

use super::MOD_PATH;
use crate::LIB_PATH;

const CLASS_PATH : &str = "service";

use super::characteristic_specification::CharacteristicSpecification;

/// Service Specification
#[derive(Clone,Default,Debug,Deserialize, HasId, HasName, HasLastUpdate, Serialize)]
pub struct ServiceSpecification {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id : Option<String>,
    /// HREF 
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href : Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name : Option<String>,
    /// Last Update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update : Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    /// Is this specification part of a bundle?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    /// Status of this specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    /// Validity period
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for : Option<TimePeriod>,
    /// Versioning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Characteristics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec_characteristics: Option<Vec<CharacteristicSpecification>>,
    /// Related Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
}

impl ServiceSpecification {
    /// Create a new specification
    pub fn new(name : impl Into<String>) -> ServiceSpecification {
        let mut ss = ServiceSpecification::create_with_time();
        ss.name = Some(name.into());
        ss.spec_characteristics = Some(vec![]);
        ss.is_bundle = Some(false);
        ss.lifecycle_status = Some("New".to_string());
        ss
    }

    /// Add a characteristic to this service specification
    pub fn add_char(&mut self, characteristic : CharacteristicSpecification) {
        self.spec_characteristics.as_mut().unwrap().push(characteristic);
    }
}

/// Reference to Service Specification
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct ServiceSpecificationRef {
    id : String,
    href : String,
    name: String,
    version: Option<String>,
}

impl From<ServiceSpecification> for ServiceSpecificationRef {
    fn from(value: ServiceSpecification) -> Self {
        ServiceSpecificationRef {
            id : value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
            version: value.version.clone(),
        }
    }
}