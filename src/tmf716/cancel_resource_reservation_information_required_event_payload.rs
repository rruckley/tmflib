use serde::{Serialize, Deserialize};
use super::CancelResourceReservation;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelResourceReservationInformationRequiredEventPayload {
    #[serde(rename = "cancelResourceReservation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_resource_reservation: Option<CancelResourceReservation>,
}
impl std::fmt::Display for CancelResourceReservationInformationRequiredEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
