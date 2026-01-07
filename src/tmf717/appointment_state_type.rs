use serde::{Serialize, Deserialize};
///Valid values for the lifecycle state of the appointment
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AppointmentStateType {
    #[serde(rename = "initialized")]
    Initialized,
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
