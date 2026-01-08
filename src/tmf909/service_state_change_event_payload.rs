use serde::{Serialize, Deserialize};
use super::Service;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStateChangeEventPayload {
    ///Service is a base class for defining the Service hierarchy. All Services are characterized as either being possibly visible and usable by a Customer or not. This gives rise to the two subclasses of Service: CustomerFacingService and ResourceFacingService.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}
impl std::fmt::Display for ServiceStateChangeEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
