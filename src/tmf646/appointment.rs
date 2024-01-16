//! Appointment Booking Module

use serde::{Deserialize, Serialize};

use super::MOD_PATH;

use crate::{HasId, CreateTMF, HasLastUpdate, CreateTMFWithTime, LIB_PATH, TimePeriod};
use tmflib_derive::{HasId,HasLastUpdate};

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
#[derive(Clone, Debug, Default, Deserialize, HasId, HasLastUpdate, Serialize)]
pub struct Appointment {
    id: Option<String>,
    href: Option<String>,
    category: Option<String>,
    creation_date: Option<String>,
    description: Option<String>,
    external_id: Option<String>,
    last_update: Option<String>,
    status: AppointmentStateType,
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

