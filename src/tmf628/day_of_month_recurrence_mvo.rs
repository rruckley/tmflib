use serde::{Serialize, Deserialize};
use crate::common::extensible::Extensible;
use crate::DateTime;

///Day Of Month Recurrence MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DayOfMonthRecurrenceMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Dates of the month for the recurrence
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<DateTime>,
}
impl std::fmt::Display for DayOfMonthRecurrenceMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DayOfMonthRecurrenceMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for DayOfMonthRecurrenceMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
