use serde::{Serialize, Deserialize};
use super::{ExtensibleFvo, PolicyVariableRefOrValueFvo};

///PolicyOperatorFvo represents a policy operator with an extensible schema and an optional policy variable.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyOperatorFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///Container for PolicyVariable Reference or unmanaged PolicyVariable object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable: Option<PolicyVariableRefOrValueFvo>,
}
impl std::fmt::Display for PolicyOperatorFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PolicyOperatorFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for PolicyOperatorFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
