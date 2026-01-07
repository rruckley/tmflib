use serde::{Serialize, Deserialize};
use super::{ServiceProblemRef, TrackingRecord};
/**Task resource that requests acknowledgement of problems by the problem handler.
Skipped properties: id,href*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProblemAcknowledgementCreate {
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
    ///The problems that were acknowledged, populated in the output to this task
    #[serde(rename = "ackProblem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_problem: Option<Vec<ServiceProblemRef>>,
    ///The problems to be acknowledged, relevant in the input to this task
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub problem: Vec<ServiceProblemRef>,
    ///Tracking records allow the tracking of modifications on the problem. The tracking records should not be embedded in the problem to allow retrieving the problem without the tracking records
    #[serde(rename = "trackingRecord")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_record: Option<TrackingRecord>,
}
impl std::fmt::Display for ProblemAcknowledgementCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
