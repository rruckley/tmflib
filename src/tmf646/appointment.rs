//! Appointment Booking Module

use serde::{Deserialize, Serialize};

use super::MOD_PATH;

use crate::{HasId, HasLastUpdate, LIB_PATH, HasValidity, TimePeriod, DateTime};
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
    creation_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<DateTime>,
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
        //let appointment = 
        Appointment::create_with_time()
        //appointment
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const APPOINTMENT_JSON : &str = "{
        \"description\" : \"AppointmentDescription\",
        \"id\" : \"AP123\",
        \"href\" : \"http://example.com/tmf646/appointment/AP123\",
        \"status\" : \"Initialized\"
    }";
    const APPOINTMENTREF_JSON : &str = "{
        \"description\" : \"AppointmentDescription\",
        \"id\" : \"AP123\",
        \"href\" : \"http://example.com/tmf646/appointment/AP123\"
    }";
    const APPOINTMENTTYPE_JSON: &str = "\"Initialized\"";

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

    #[test]
    fn test_ref_from_appointment() {
        let appointment = Appointment::new();

        let appoint_ref = AppointmentRef::from(appointment.clone());

        assert_eq!(appointment.get_id(),appoint_ref.id);
        assert_eq!(appointment.get_href(),appoint_ref.href); 
    }

    #[test]
    fn test_appointmentstate_deserialize() {
        let appointmentstate : AppointmentStateType = serde_json::from_str(APPOINTMENTTYPE_JSON).unwrap();

        assert_eq!(appointmentstate,AppointmentStateType::Initialized);
    }

    #[test]
    fn test_appointment_deserialize() {
        let appointment : Appointment = serde_json::from_str(APPOINTMENT_JSON).unwrap();

        assert_eq!(appointment.get_id(),"AP123");
        assert_eq!(appointment.description.unwrap().as_str(),"AppointmentDescription");
        // assert_eq!(appointment.get_href(),"http://example.com/tmf646/appointment/AP123");
        assert_eq!(appointment.status,AppointmentStateType::Initialized);
    }

    #[test]
    fn test_appointmentref_deserialize() {
        let appointmentref : AppointmentRef = serde_json::from_str(APPOINTMENTREF_JSON).unwrap();

        assert_eq!(appointmentref.id.as_str(),"AP123");
        assert_eq!(appointmentref.description.as_str(),"AppointmentDescription");
        assert_eq!(appointmentref.href.as_str(),"http://example.com/tmf646/appointment/AP123");
    }

    #[test]
    fn test_appointment_validity() {
        let mut appointment = Appointment::new();

        appointment.set_validity(TimePeriod::period_30days());

        assert_eq!(appointment.valid_for.is_some(),true);
        let validity = appointment.get_validity().unwrap();
        assert_eq!(validity.started(),true);
        assert_eq!(validity.finished(),false);
    }
}

