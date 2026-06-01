use serde::{Deserialize, Serialize};
// use super::EntityRef;
use crate::common::entity::EntityRef;

///`ConstraintRef` represents a reference to a constraint, which may include the version of the constraint. It is used to define the reference to a constraint in a specific context, e.g. for a specific customer or in a specific environment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstraintRefFvo {
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(flatten)]
    pub entity_ref: EntityRef,
    ///constraint version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ConstraintRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ConstraintRefFvo {
    type Target = EntityRef;
    fn deref(&self) -> &Self::Target {
        &self.entity_ref
    }
}
impl std::ops::DerefMut for ConstraintRefFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_ref
    }
}
