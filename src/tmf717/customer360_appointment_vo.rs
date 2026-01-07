use serde::{Serialize, Deserialize};
use super::{AppointmentStateType, CalendarEventRef, Entity, TimePeriod};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer360AppointmentVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Reference of a CalendarEvent
    #[serde(rename = "calendarEvent")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_event: Option<CalendarEventRef>,
    ///Business category : intervention for example or to be more precise after SalesIntervention, orderDeliveryIntervention,...
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    ///Appointment creation date
    #[serde(rename = "creationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<crate::DateTime>,
    ///Short free text describing the appointment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///External reference known by the Partner
    #[serde(rename = "externalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    ///Date of last appointment update
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Valid values for the lifecycle state of the appointment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AppointmentStateType>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for Customer360AppointmentVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360AppointmentVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360AppointmentVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
