use serde::{Serialize, Deserialize};
///Valid values for the lifecycle state of the appointment
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AppointmentStateType {
    ///Appointment has been created but not yet confirmed
    #[serde(rename = "initialized")]
    Initialized,
    ///Appointment has been confirmed
    #[serde(rename = "confirmed")]
    Confirmed,
    ///Appointment has been cancelled
    #[serde(rename = "cancelled")]
    Cancelled,
    ///Appointment has been completed
    #[serde(rename = "completed")]
    Completed,
    ///Appointment has failed
    #[serde(rename = "failed")]
    Failed,
}
