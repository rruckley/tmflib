use crate::common::extensible::Extensible;
use crate::TimePeriod;
use serde::{Deserialize, Serialize};

///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgreementTermOrConditionMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Text that explains the term or condition of the agreement.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Unique number assigned for reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for AgreementTermOrConditionMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AgreementTermOrConditionMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for AgreementTermOrConditionMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
