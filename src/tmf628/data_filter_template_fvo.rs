use crate::common::extensible::ExtensibleFvo;
use serde::{Deserialize, Serialize};

///Data Filter Template FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataFilterTemplateFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///Description of this DataFilterTemplate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name given to this DataFilterTemplate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for DataFilterTemplateFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DataFilterTemplateFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for DataFilterTemplateFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
