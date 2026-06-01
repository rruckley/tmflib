use super::ResourceReservation;
use serde::{Deserialize, Serialize};
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceReservationDeleteEventPayload {
    /// `ResourceReservationDeleteEventPayload`
    #[serde(rename = "resourceReservation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_reservation: Option<ResourceReservation>,
}
impl std::fmt::Display for ResourceReservationDeleteEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
