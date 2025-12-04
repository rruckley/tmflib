use super::ExtensibleFvo;
use serde::{Deserialize, Serialize};

///Note structure to capture user comments
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NoteFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///Author of the note
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    ///Date of the note
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::DateTime>,
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
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for NoteFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
