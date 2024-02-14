//! Service Module

use serde::{Deserialize,Serialize};

use super::MOD_PATH;
use crate::common::related_party::RelatedParty;
use crate::common::note::Note;
use crate::{CreateTMF, DateTime, HasId, HasName, TimePeriod, HasNote, LIB_PATH};
use tmflib_derive::{HasId, HasName, HasNote};

const CLASS_PATH : &str = "service";

#[derive(Clone,Debug,Default,Deserialize,PartialEq,Serialize)]
#[serde(rename_all = "camelCase")]
enum ServiceStateType {
    FeasibilityChecked,
    Designed,
    Reserved,
    #[default]
    Inactive,
    Active,
    Terminated,
}

#[derive(Clone,Debug,Default,Deserialize,PartialEq,Serialize)]
#[serde(rename_all = "camelCase")]

/// Service Features
pub struct Feature {
    id : String,
    is_bundle: bool,
    is_enabled: bool,
    name: String,
    feature_relationship: Option<Vec<FeatureRelationship>>,
}

/// Feature Relationships
#[derive(Clone,Debug,Default,Deserialize,PartialEq,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRelationship {
    id : String,
    name: String,
    relationship_type: String,
    valid_for: TimePeriod,
}

/// Service record from the Service Inventory
#[derive(Clone,Debug,Default,Deserialize, HasId, HasName, HasNote, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    category: Option<String>,
    description: Option<String>,
    end_date: Option<DateTime>,
    has_started: Option<bool>,
    id: Option<String>,
    href: Option<String>,
    is_bundle: Option<bool>,
    is_service_enabled: Option<bool>,
    is_stateful: Option<bool>,
    name: Option<String>,
    service_date: Option<DateTime>,
    service_type: Option<String>,
    start_date: Option<DateTime>,
    start_mode: Option<String>,
    state : ServiceStateType,
    // Referenced fields
    related_party: Option<Vec<RelatedParty>>,
    note: Option<Vec<Note>>,
    feature: Option<Vec<Feature>>,
}

impl Service {
    /// Create a new service object for the inventory
    pub fn new(name : impl Into<String>) -> Service {
        let mut service = Service::create();
        service.name = Some(name.into());
        service.is_bundle = Some(false);
        service
    }
}

#[cfg(test)]
mod test {
    use crate::tmf638::service::ServiceStateType;
    use crate::HasName;

    const SERVICE: &str = "AService";

    use super::Service;
    #[test]
    fn test_service_create_name() {
        let service = Service::new(SERVICE);

        assert_eq!(service.get_name(),SERVICE.to_string());
    }

    #[test]
    fn test_service_default_state() {
        let service = Service::default();

        assert_eq!(service.state , ServiceStateType::Inactive);
    }

    #[test]
    fn test_service_new_bundle() {
        let service = Service::new(SERVICE);

        assert_eq!(service.is_bundle,Some(false));
    }
}