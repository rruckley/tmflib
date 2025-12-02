use serde::{Serialize, Deserialize};
use super::{AgreementTermOrConditionMvo, Extensible};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgreementItemMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Identifier of the Agreement item (generally it is a sequence number 01, 02, 03, ...)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "termOrCondition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub term_or_condition: Vec<AgreementTermOrConditionMvo>,
}
impl std::fmt::Display for AgreementItemMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AgreementItemMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for AgreementItemMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
