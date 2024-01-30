//! Incident Module
//! 

use serde::{Deserialize,Serialize};
use crate::{LIB_PATH,HasId,CreateTMF, HasName, Uri};
use tmflib_derive::{HasId,HasName};
use super::MOD_PATH;
const CLASS_PATH : &str = "incident";

/// Incident Priority
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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
pub enum IncidentAckStateType {
    /// Acknowledged
    Acknowledged,
    /// Unacknowledged
    #[default]
    Unacknowledged,
}

/// Incident Status
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

/// ITIL Incident
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Incident {
    city: String,
    country : String,
    geographic_address_type: String,
    /// Url to Incident
    pub href: Option<Uri>,
    /// Id for incident
    pub id: Option<String>,
    /// Incident Title
    pub name: Option<String>,
}