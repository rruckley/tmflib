use serde::{Serialize, Deserialize};
use super::Extensible;
use crate::DateTime;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DayOfMonthRecurrence {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<DateTime>,
}
impl std::fmt::Display for DayOfMonthRecurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DayOfMonthRecurrence {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for DayOfMonthRecurrence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
