//! Incident Module
//! 

use serde::{Deserialize,Serialize};
use crate::{LIB_PATH,HasId,CreateTMF, HasName, Uri, DateTime};
use tmflib_derive::{HasId,HasName};
use super::MOD_PATH;
const CLASS_PATH : &str = "incident";

/// Incident Priority
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PriorityType {
    /// Critical
    Critical,
    /// High
    High,
    /// Medium [default]
    #[default]
    Medium,
    /// Low
    Low,
}

/// Incident Urgency 
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum IncidentAckStateType {
    /// Acknowledged
    Acknowledged,
    /// Unacknowledged
    #[default]
    Unacknowledged,
}

/// Incident Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ImpactType {
    /// Extensive [Highest]
    Extensive,
    /// Significant
    Significant,
    /// Moderate [default]
    #[default]
    Moderate,
    /// Minor
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
    pub value : String,
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

/// External Identifier Reference
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIdentifier {
    external_identifier_type: String,
    href: Uri,
    id: String,
    owner: String,
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
    pub extension_info: Option<Vec<Characteristic>>,
    /// Events
    pub event_id: Option<Vec<ResourceEntity>>,
    /// Root Event
    pub root_event_id: Option<Vec<ResourceEntity>>,
    /// Source Object
    pub source_object: Option<Vec<ResourceEntity>>,
    /// Root Cause(s)
    pub root_cause: Option<Vec<RootCause>>,
    /// Affected Entities
    pub affected_entity: Option<Vec<EntityRef>>,
    /// External Identifiers
    pub external_identifier: Option<Vec<ExternalIdentifier>>,
}