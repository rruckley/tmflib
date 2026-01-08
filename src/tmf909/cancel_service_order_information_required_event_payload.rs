use serde::{Serialize, Deserialize};
use super::CancelServiceOrder;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelServiceOrderInformationRequiredEventPayload {
    ///Request for cancellation an existing Service order
    #[serde(rename = "cancelServiceOrder")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_service_order: Option<CancelServiceOrder>,
}
impl std::fmt::Display for CancelServiceOrderInformationRequiredEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
