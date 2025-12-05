use serde::{Serialize, Deserialize};
use super::{Extensible, HourPeriodMvo};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CalendarPeriodMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Day where the calendar status applies (e.g.: monday, mon-to-fri, weekdays, weekend, all week, ...)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    ///Collection of hour intervals
    #[serde(rename = "hourPeriod")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hour_period: Vec<HourPeriodMvo>,
    ///Indication of the availability of the calendar period (e.g.: available, booked, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///Indication of the timezone applicable to the calendar information (e.g.: Paris, GMT+1)
    #[serde(rename = "timeZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
impl std::fmt::Display for CalendarPeriodMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CalendarPeriodMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for CalendarPeriodMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
