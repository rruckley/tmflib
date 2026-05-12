use super::EntityRefMvo;
use serde::{Deserialize, Serialize};

///ConstraintRefMvo represents a reference to a constraint, which may include the version of the constraint. It is used to define the reference to a constraint in a specific context, e.g. for a specific product or in a specific environment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstraintRefMvo {
    ///Base schema for addressable entities
    #[serde(flatten)]
    pub entity_ref_mvo: EntityRefMvo,
    ///constraint version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ConstraintRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ConstraintRefMvo {
    type Target = EntityRefMvo;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref_mvo
    }
}
impl std::ops::DerefMut for ConstraintRefMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref_mvo
    }
}
