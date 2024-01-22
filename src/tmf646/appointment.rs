//! Appointment Booking Module

use serde::{Deserialize, Serialize};

use super::MOD_PATH;

use crate::{HasId, CreateTMF, HasLastUpdate, CreateTMFWithTime, LIB_PATH, HasValidity, TimePeriod};
use tmflib_derive::{HasId,HasLastUpdate, HasValidity};

const CLASS_PATH : &str = "appointment";

/// Appointment booking status
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum AppointmentStateType {
    /// Appointment has been initialized
    #[default]
    Initialized,
    /// Appointment has been confirmed
    Confirmed,
    /// Appointment has been cancelled
    Cancelled,
    /// Appointment booking has been completed, i.e. scheduled time has passed.
    Completed,
    /// Appointment booking failed.
    Failed,
}

/// Appointment booking
#[derive(Clone, Debug, Default, Deserialize, HasId, HasLastUpdate, HasValidity, Serialize)]
pub struct Appointment {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<String>,
    status: AppointmentStateType,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
}

/// Reference to an appointment
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct AppointmentRef {
    description: String,
    href: String,
    id: String,
}

impl From<Appointment> for AppointmentRef {
    fn from(value: Appointment) -> Self {
        AppointmentRef {
            description: value.get_href(),
            href: value.href.unwrap().clone(),
            id: value.id.unwrap().clone(),
        }
    }
}

impl Appointment {
    /// Create new appointment record
    pub fn new() -> Appointment {
        let appointment = Appointment::create_with_time();
        appointment
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_appointment_id() {
        let appointment = Appointment::new();

        assert_eq!(appointment.id.is_some(),true);
        assert_eq!(appointment.href.is_some(),true);
    }

    #[test]
    fn test_create_appointment_status() {
        let appointment = Appointment::new();

        assert_eq!(appointment.status, AppointmentStateType::Initialized);
    }
}

