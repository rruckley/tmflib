use serde::{Serialize, Deserialize};
use super::{Extensible, PolicyVariableRefOrValueMvo};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyOperatorMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Container for PolicyVariable Reference or unmanaged PolicyVariable object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable: Option<PolicyVariableRefOrValueMvo>,
}
impl std::fmt::Display for PolicyOperatorMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PolicyOperatorMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for PolicyOperatorMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
