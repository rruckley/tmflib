use serde::{Serialize, Deserialize};
use super::{AgreementTermOrConditionFvo, ExtensibleFvo};

/// An item of an agreement
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgreementItemFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///Identifier of the Agreement item (generally it is a sequence number 01, 02, 03, ...)
    pub id: String,
    ///Terms or conditions associated with the agreement item
    #[serde(rename = "termOrCondition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub term_or_condition: Vec<AgreementTermOrConditionFvo>,
}
impl std::fmt::Display for AgreementItemFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AgreementItemFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for AgreementItemFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
