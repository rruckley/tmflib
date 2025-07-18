//! Incident Module
//! 

use serde::{Deserialize,Serialize};
use crate::{LIB_PATH,HasId, HasName, Uri, DateTime};
use crate::common::external_identifier::ExternalIdentifier;
use tmflib_derive::{HasId,HasName};
use super::MOD_PATH;
const CLASS_PATH : &str = "incident";

/// Incident Priority
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum PriorityType {
    /// Critical
    Critical,
    /// High
    High,
    /// Medium (Default)
    #[default]
    Medium,
    /// Low
    Low,
}

/// Incident Urgency 
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum UrgencyType {
    /// Critical
    Critical,
    /// High
    High,
    /// Medium
    #[default]
    Medium,
    /// Low
    Low,
}

/// Incident Acknowledge State
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum IncidentAckStateType {
    /// Acknowledged
    Acknowledged,
    /// Unacknowledged
    #[default]
    Unacknowledged,
}

/// Incident Status
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum IncidentStateType {
    /// Raised
    #[default]
    Raised,
    /// Updated
    Updated,
    /// Cleared
    Cleared,
}

/// Incident Impact
#[derive(Clone, Debug, Default, Deserialize,PartialEq, Serialize)]
pub enum ImpactType {
    /// Extensive (Highest)
    Extensive,
    /// Significant
    Significant,
    /// Moderate
    #[default]
    Moderate,
    /// Minor (Lowest)
    Minor,
}

/// Incident Impact
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    /// Unique Id
    pub id : String,
    /// Name
    pub name : String,
    /// Value
    pub value : serde_json::Value,
    /// Value type
    pub value_type: String,
}

/// Reference to an external reource
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceEntity {
    /// Referenced Uri
    pub href: Option<Uri>,
    /// Referenced Id
    pub id: Option<String>,
}
/// Root Cause
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RootCause {
    href : Uri,
    id : String,
    location: String,
}

/// Entity Reference
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityRef {
    href: Uri,
    id : String,
    name: String,
}



/// ITIL Incident
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Incident {
    /// Acknowledged Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ack_state: Option<IncidentAckStateType>,
    /// Acknowledged Time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ack_time: Option<DateTime>,
    /// Incident Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Incident resolution time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_time: Option<DateTime>,
    /// Domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Url to Incident
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Id for incident
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// What is the impact of this incident?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<ImpactType>,
    /// Detailed description of incident
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_detail: Option<String>,
    /// Suggested resolution for this incident
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_resolution_suggestion: Option<String>,
    /// Incident Title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Incident first occurance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occur_time: Option<DateTime>,
    /// Priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<PriorityType>,
    /// Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<IncidentStateType>,
    /// Last update time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<DateTime>,
    /// Urgency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgency: Option<UrgencyType>,

    // Referenced types
    /// Extensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_info: Option<Vec<Characteristic>>,
    /// Events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<Vec<ResourceEntity>>,
    /// Root Event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_event_id: Option<Vec<ResourceEntity>>,
    /// Source Object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_object: Option<Vec<ResourceEntity>>,
    /// Root Cause(s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_cause: Option<Vec<RootCause>>,
    /// Affected Entities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_entity: Option<Vec<EntityRef>>,
    /// External Identifiers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_identifier: Option<Vec<ExternalIdentifier>>,
}

impl Incident {
    /// Create a new incident
    pub fn new(name : impl Into<String>) -> Incident {
        Incident {
            name: Some(name.into()),
            impact: Some(ImpactType::Moderate),
            priority: Some(PriorityType::Medium),
            urgency: Some(UrgencyType::Medium),
            ack_state: Some(IncidentAckStateType::Unacknowledged),
            state: Some(IncidentStateType::Raised),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PRIORITY_JSON : &str = "\"Medium\"";
    const URGENCY_JSON : &str = "\"Medium\"";
    const ACKSTATE_JSON : &str = "\"Unacknowledged\"";
    const STATE_JSON : &str = "\"Raised\"";
    const IMPACT_JSON : &str = "\"Moderate\"";

    const CHAR_JSON : &str = "{
        \"id\" : \"CHAR123\",
        \"name\" :\"CharacteristicName\",
        \"value\" : \"Value\",
        \"valueType\" : \"ValueType\"
    }";


    #[test]
    fn test_incident_new_name() {
        let incident = Incident::new("AnIncident");

        assert_eq!(incident.name,Some("AnIncident".to_string()));
    }

    #[test]
    fn test_incident_new_priority() {
        let incident = Incident::new("AnIncident");

        assert_eq!(incident.priority,Some(PriorityType::Medium));
    }

    #[test]
    fn test_incident_new_impact() {
        let incident = Incident::new("AnIncident");
        
        assert_eq!(incident.impact,Some(ImpactType::Moderate));
    }

    #[test]
    fn test_incident_new_urgency() {
        let incident = Incident::new("AnIncident");

        assert_eq!(incident.urgency,Some(UrgencyType::Medium));
    }

    #[test]
    fn test_incident_new_state() {
        let incident = Incident::new("AnIncident");
        
        assert_eq!(incident.state,Some(IncidentStateType::Raised));
    }

    #[test]
    fn test_incident_new_ack() {
        let incident = Incident::new("AnIncident");

        assert_eq!(incident.ack_state, Some(IncidentAckStateType::Unacknowledged));
    }
    #[test]
    fn test_priority_deserialize() {
        let priority : PriorityType = serde_json::from_str(PRIORITY_JSON).unwrap();

        assert_eq!(priority,PriorityType::Medium);
    }

    #[test]
    fn test_urgency_deseralize() {
        let urgency : UrgencyType = serde_json::from_str(URGENCY_JSON).unwrap();

        assert_eq!(urgency,UrgencyType::Medium);
    }

    #[test]
    fn test_ackstate_deserialize() {
        let ackstate : IncidentAckStateType = serde_json::from_str(ACKSTATE_JSON).unwrap();

        assert_eq!(ackstate,IncidentAckStateType::Unacknowledged);
    }

    #[test]
    fn test_state_deseralize() {
        let state : IncidentStateType = serde_json::from_str(STATE_JSON).unwrap();

        assert_eq!(state,IncidentStateType::Raised);
    }

    #[test]
    fn test_impact_deseralize() {
        let impact : ImpactType = serde_json::from_str(IMPACT_JSON).unwrap();

        assert_eq!(impact, ImpactType::Moderate);
    }

    #[test]
    fn test_char_deserialize() {
        let characteristic : Characteristic = serde_json::from_str(CHAR_JSON).unwrap();

        assert_eq!(characteristic.id.as_str(),"CHAR123");
        assert_eq!(characteristic.name.as_str(),"CharacteristicName");
        assert_eq!(characteristic.value.as_str(),"Value".into()  );
        assert_eq!(characteristic.value_type.as_str(), "ValueType");
    }
}