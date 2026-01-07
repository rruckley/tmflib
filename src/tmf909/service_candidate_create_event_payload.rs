use serde::{Serialize, Deserialize};
use super::ServiceCandidate;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceCandidateCreateEventPayload {
    /**ServiceCandidate is an entity that makes a service specification available to a catalog. A
ServiceCandidate and its associated service specification may be published - made visible - in any number of service catalogs, or in none. One service specification can be composed of other service specifications.*/
    #[serde(rename = "serviceCandidate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_candidate: Option<ServiceCandidate>,
}
impl std::fmt::Display for ServiceCandidateCreateEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
