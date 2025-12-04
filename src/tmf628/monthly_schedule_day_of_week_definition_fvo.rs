use super::{DayOfMonthRecurrenceFvo, ExtensibleFvo};
use serde::{Deserialize, Serialize};

/// Monthly Schedule Day Of Week Definition Fvo
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonthlyScheduleDayOfWeekDefinitionFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    /// Day Of Month Recurrence
    #[serde(rename = "dayOfMonthRecurrence")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub day_of_month_recurrence: Vec<DayOfMonthRecurrenceFvo>,
    /// Recurring Day Sequence
    #[serde(rename = "recurringDaySequence")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurring_day_sequence: Option<String>,
}
impl std::fmt::Display for MonthlyScheduleDayOfWeekDefinitionFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for MonthlyScheduleDayOfWeekDefinitionFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for MonthlyScheduleDayOfWeekDefinitionFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
