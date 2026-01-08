use serde::{Serialize, Deserialize};
use super::ServiceTest;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceTestCreateEventPayload {
    /**A service test is an entity that exists for a controlled test invocation on a service. The service
test is executed according to a schedule and contains service test configuration parameters that are to be
applied at execution time, and service test measures that result.*/
    #[serde(rename = "serviceTest")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_test: Option<ServiceTest>,
}
impl std::fmt::Display for ServiceTestCreateEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
