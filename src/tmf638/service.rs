//! Service Module

use serde::{Deserialize,Serialize};

use super::MOD_PATH;
use crate::common::related_party::RelatedParty;
use crate::common::note::Note;
use crate::{DateTime, HasId, HasName, HasDescription, TimePeriod, HasNote, LIB_PATH, vec_insert,serde_value_to_type};
use tmflib_derive::{HasId, HasName, HasNote, HasDescription};

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

/// Service Features
#[derive(Clone,Debug,Default,Deserialize,PartialEq,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    id : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_bundle: Option<bool>,
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

/// Service Characteristics
/// Characteristics are used to describe the service in more detail.
#[derive(Clone,Debug,Default,Deserialize,PartialEq,Serialize)]
pub struct Characteristic {
    id: Option<String>,
    name: String,
    value: Option<serde_json::Value>,
    value_type: Option<String>,
}   

impl Characteristic {
    /// Create a new characteristic with a given name and value, value_type is determined automatically based on value enum.
    pub fn new(name : String, value : serde_json::Value) -> Characteristic {
        let val_type = serde_value_to_type(&value);
        Characteristic { id: None, name: name, value: Some(value.clone()), value_type: Some(val_type.to_string()) }
    }
}

/// Service Relationships
/// Relationships are used to describe how services relate to each other.
/// For example, a service may depend on another service or be a part of a bundle.

#[derive(Clone,Debug,Default,Deserialize,PartialEq,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceRelationship {
    /// Service Relationship Type
    pub relationship_type: String,
    /// Service Relationship Characteristic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_relationship_characteristic: Option<Vec<Characteristic>>,
}

/// Service record from the Service Inventory
#[derive(Clone,Debug,Default,Deserialize, HasId, HasName,HasDescription, HasNote, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    /// Service Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Service Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    end_date: Option<DateTime>,
    has_started: Option<bool>,
    /// Service ID  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Service Href
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    is_bundle: Option<bool>,
    is_service_enabled: Option<bool>,
    is_stateful: Option<bool>,
    /// Service Name    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    service_date: Option<DateTime>,
    service_type: Option<String>,
    start_date: Option<DateTime>,
    start_mode: Option<String>,
    state : ServiceStateType,
    // Referenced fields
    related_party: Option<Vec<RelatedParty>>,
    /// Service Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    /// Service Features
    #[serde(skip_serializing_if = "Option::is_none")]
    feature: Option<Vec<Feature>>,
    /// Service Ch aracteristics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_characteristic: Option<Vec<Characteristic>>,
    /// Service Relationships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_relationship: Option<Vec<ServiceRelationship>>,
}

impl Service {
    /// Create a new service object for the inventory
    pub fn new(name : impl Into<String>) -> Service {
        let mut service = Service::create();
        service.name = Some(name.into());
        service.is_bundle = Some(false);
        service
    }

    /// Add a characterisitic during create
    pub fn with_characteristic(mut self, characteristic: Characteristic) -> Service {
        vec_insert(&mut self.service_characteristic, characteristic);
        self
    }

    /// Add relationships during create
    pub fn with_relationship(mut self, relationship: ServiceRelationship) -> Service {
        vec_insert(&mut self.service_relationship, relationship);
        self
    }

    /// Get a characteristic by name
    pub fn get_characteristics(&self, name : impl Into<String>) -> Option<Vec<Characteristic>> {
        match self.service_characteristic {
            Some(ref characteristics) => {
                let name : String = name.into();
                let out = characteristics.iter()
                    .filter(|c| c.name == name)
                    .cloned()
                    .collect();
                Some(out)
            },
            None => None,
        }
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

    #[test]
    fn test_service_characteristic_add() {
        let characteristic = super::Characteristic {
            id: "char1".to_string().into(),
            name: "Characteristic1".to_string(),
            value: Some("Value1".into()),
            value_type: Some("String".to_string()),
        };

        let service = Service::new(SERVICE).with_characteristic(characteristic);

        // assert_eq!(service.service_characteristic.unwrap().len(), 1);
        assert_eq!(service.service_characteristic.unwrap()[0].name, "Characteristic1");
    }   

    #[test]
    fn test_service_characteristic_get() {
        let characteristic = super::Characteristic {
            id: "char1".to_string().into(),
            name: "Characteristic1".to_string(),
            value: Some("Value1".into()),
            value_type: Some("String".to_string()),
        };
        let service = Service::new(SERVICE).with_characteristic(characteristic);   
        let characteristics = service.get_characteristics("Characteristic1");
        assert!(characteristics.is_some());
        let characteristics = characteristics.unwrap();
        assert_eq!(characteristics.len(), 1);
        assert_eq!(characteristics[0].name, "Characteristic1");
    }

    #[test]
    fn test_service_relationship_add() {
        let relationship = super::ServiceRelationship {
            relationship_type: "DependsOn".to_string(),
            service_relationship_characteristic: Some(vec![super::Characteristic {
                id: "rel1".to_string().into(),
                name: "Relationship1".to_string(),
                value: Some("Value1".into()),
                value_type: Some("String".to_string()),
            }]),
        }; 
        let service = Service::new(SERVICE).with_relationship(relationship);
        assert!(service.service_relationship.is_some());
        assert_eq!(service.service_relationship.unwrap().len(), 1);
        // assert_eq!(service.service_relationship.unwrap()[0].relationship_type, "DependsOn");
    }
}