//! Appointment Booking Module

use serde::{Deserialize, Serialize};

use super::MOD_PATH;

use crate::{HasId, CreateTMF, HasLastUpdate, CreateTMFWithTime, LIB_PATH};

const APP_PATH : &str = "appointment";

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
    creation_date: Option<String>,
    last_update: Option<String>,
    status: AppointmentStateType,
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
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,APP_PATH,self.get_id());
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
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
    fn get_class() -> String {
        APP_PATH.to_owned()
    }
}

impl CreateTMF<Appointment> for Appointment {}

impl HasLastUpdate for Appointment {
    fn set_last_update(&mut self, time : String) {
        self.last_update = Some(time.clone());
        self.creation_date = Some(time);
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

