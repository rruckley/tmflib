use super::ChangeRequest;
use serde::{Deserialize, Serialize};
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRequestStatusChangeEventPayload {
    /**Change Request is a type of request which can be used for the management and control of Change Management process
    -within a service provider organisation or
    -between a customer and a service provider or
    -between a service provider and a partner and vice versa.*/
    #[serde(rename = "changeRequest")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change_request: Option<ChangeRequest>,
}
impl std::fmt::Display for ChangeRequestStatusChangeEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
