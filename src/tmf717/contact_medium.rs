use serde::{Serialize, Deserialize};
use super::{Extensible, TimePeriod};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContactMedium {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Type of the contact medium to qualify it like pro email / personal email. This is not used to define the contact medium used.
    #[serde(rename = "contactType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<String>,
    ///Identifier for this contact medium.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///If true, indicates that is the preferred contact medium
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred: Option<bool>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for ContactMedium {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ContactMedium {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for ContactMedium {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
