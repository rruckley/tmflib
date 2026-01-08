use serde::{Serialize, Deserialize};
use super::ServiceProblem;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceProblemCreateEventPayload {
    ///The problem information for Middle B which is abstracted in the service layer from the issued event information by First B
    #[serde(rename = "serviceProblem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_problem: Option<ServiceProblem>,
}
impl std::fmt::Display for ServiceProblemCreateEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
