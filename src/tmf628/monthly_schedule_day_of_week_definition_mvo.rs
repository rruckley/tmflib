use super::DayOfMonthRecurrenceMvo;
use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

/// Monthly Schedule Day Of Week Definition Mvo
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonthlyScheduleDayOfWeekDefinitionMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    /// Day Of Month Recurrence
    #[serde(rename = "dayOfMonthRecurrence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub day_of_month_recurrence: Vec<DayOfMonthRecurrenceMvo>,
    /// Recurring Day Sequence
    #[serde(rename = "recurringDaySequence")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_day_sequence: Option<String>,
}
impl std::fmt::Display for MonthlyScheduleDayOfWeekDefinitionMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for MonthlyScheduleDayOfWeekDefinitionMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for MonthlyScheduleDayOfWeekDefinitionMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
