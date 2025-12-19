use crate::common::extensible::ExtensibleFvo;
use crate::TimePeriod;
use serde::{Deserialize, Serialize};
// use crate::common::extensible::Extensible;
/// A term or condition of an agreement
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgreementTermOrConditionFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
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
impl std::fmt::Display for AgreementTermOrConditionFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AgreementTermOrConditionFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for AgreementTermOrConditionFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
