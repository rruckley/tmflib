use super::CancelResourceReservation;
use serde::{Deserialize, Serialize};
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelResourceReservationStateChangeEventPayload {
    /// `CancelResourceReservationStateChangeEventPayload`
    #[serde(rename = "cancelResourceReservation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_resource_reservation: Option<CancelResourceReservation>,
}
impl std::fmt::Display for CancelResourceReservationStateChangeEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
