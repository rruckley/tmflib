use serde::{Serialize, Deserialize};
use super::{Any, ServiceProblemRef};
///A record of an event (related to a service problem) received from another system
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceProblemEventRecord {
    ///Time at which the event occurred
    #[serde(rename = "eventTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<crate::DateTime>,
    ///Type of the recorded event
    #[serde(rename = "eventType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    ///reference to this resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Identifier of the service problem event record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification: Option<Any>,
    ///Time at which the record was created
    #[serde(rename = "recordTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_time: Option<crate::DateTime>,
    #[serde(rename = "serviceProblem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_problem: Option<ServiceProblemRef>,
}
impl std::fmt::Display for ServiceProblemEventRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
