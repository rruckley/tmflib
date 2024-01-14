//! Appointment Booking Module

use serde::{Deserialize, Serialize};

use super::MOD_PATH;

use crate::{HasId, CreateTMF, HasLastUpdate, CreateTMFWithTime, LIB_PATH, TimePeriod};

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
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

impl HasId for Appointment {
    fn generate_href(&mut self) {
        let href = format!("{}/{}",Appointment::get_class_href(),self.get_id());
        self.href = Some(href);       
    }
    fn generate_id(&mut self) {
        let id = Appointment::get_uuid();
        self.id = Some(id);  
        self.generate_href();
    }
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()
    }
    fn get_class_href() -> String {
        format!("/{}/{}/{}",LIB_PATH,MOD_PATH,Appointment::get_class())
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
    fn get_class() -> String {
        CLASS_PATH.to_owned()
    }
}

impl CreateTMF<Appointment> for Appointment {}

impl HasLastUpdate for Appointment {
    fn set_last_update(&mut self, time : impl Into<String>) {
        let time1 = time.into();
        let time2 = time1.clone();
        self.last_update = Some(time1);
        self.creation_date = Some(time2);
    }
}

impl CreateTMFWithTime<Appointment> for Appointment {}

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

