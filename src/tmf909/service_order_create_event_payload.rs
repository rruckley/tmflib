use serde::{Serialize, Deserialize};
use super::ServiceOrder;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOrderCreateEventPayload {
    #[serde(rename = "serviceOrder")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order: Option<ServiceOrder>,
}
impl std::fmt::Display for ServiceOrderCreateEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
