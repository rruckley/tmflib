use serde::{Serialize, Deserialize};
use super::ServiceTestSpecification;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceTestSpecificationAttributeValueChangeEventPayload {
    /**The service test specification describes the service test in terms of parameters to be configured and
measures to be taken.*/
    #[serde(rename = "serviceTestSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_test_specification: Option<ServiceTestSpecification>,
}
impl std::fmt::Display for ServiceTestSpecificationAttributeValueChangeEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
