use serde::{Serialize, Deserialize};
use super::{DayOfMonthRecurrence, Extensible};

///Monthly Schedule Day of Week Definition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonthlyScheduleDayOfWeekDefinition {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Day of month recurrence definitions
    #[serde(rename = "dayOfMonthRecurrence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub day_of_month_recurrence: Vec<DayOfMonthRecurrence>,
    ///Recurring day sequence
    #[serde(rename = "recurringDaySequence")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_day_sequence: Option<String>,
}
impl std::fmt::Display for MonthlyScheduleDayOfWeekDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for MonthlyScheduleDayOfWeekDefinition {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for MonthlyScheduleDayOfWeekDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
