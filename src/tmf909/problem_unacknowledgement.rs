use serde::{Serialize, Deserialize};
use super::{ServiceProblemRef, TrackingRecord};
///Task resource that requests unacknowledgement of problems, rolling back the status of the problems from Acknowledged to Submitted.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProblemUnacknowledgement {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Reference to this task resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Unique identifier of this task resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///The problems to be unacknowledged, relevant in the input to this task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub problem: Option<Vec<ServiceProblemRef>>,
    ///Tracking records allow the tracking of modifications on the problem. The tracking records should not be embedded in the problem to allow retrieving the problem without the tracking records
    #[serde(rename = "trackingRecord")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_record: Option<TrackingRecord>,
    ///The problems that were unacknowledged, populated in the output to this task
    #[serde(rename = "unackProblem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unack_problem: Option<Vec<ServiceProblemRef>>,
}
impl std::fmt::Display for ProblemUnacknowledgement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
