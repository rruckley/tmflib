use serde::{Serialize, Deserialize};
use super::ExtensibleFvo;
use crate::DateTime;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DayOfWeekRecurrenceFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<DateTime>,
}
impl std::fmt::Display for DayOfWeekRecurrenceFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DayOfWeekRecurrenceFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for DayOfWeekRecurrenceFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
