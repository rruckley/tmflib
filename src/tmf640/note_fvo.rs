use serde::{Serialize, Deserialize};
use crate::common::extensible::Extensible;

/// NoteFvo
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NoteFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Author of the note
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    ///Date of the note
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::DateTime>,
    ///Identifier of the note within its containing entity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Text of the note
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl std::fmt::Display for NoteFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for NoteFvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for NoteFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
