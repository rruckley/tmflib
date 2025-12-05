use serde::{Serialize, Deserialize};
use crate::common::extensible::Extensible;

/// Hour Period
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HourPeriodMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///The time when the status ends applying
    #[serde(rename = "endHour")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_hour: Option<String>,
    ///The time when the status starts applying
    #[serde(rename = "startHour")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_hour: Option<String>,
}
impl std::fmt::Display for HourPeriodMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for HourPeriodMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for HourPeriodMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
