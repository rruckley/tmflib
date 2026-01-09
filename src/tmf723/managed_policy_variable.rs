use serde::{Serialize, Deserialize};
use super::{PolicyManagedEntity, PolicyVariable};
///Managed PolicyVariable
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyVariable {
    /// Inlined fields from PolicyManagedEntity
    #[serde(flatten)]
    pub policy_managed_entity: PolicyManagedEntity,
    ///A PolicyVariable is an entity for modeling different types of variables that can be used to form a PolicyCondition statement. It can be static or dynamic.
    #[serde(flatten)]
    pub policy_variable: PolicyVariable,
}
impl std::fmt::Display for ManagedPolicyVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyVariable {
    type Target = PolicyManagedEntity;
    fn deref(&self) -> &Self::Target {
        &self.policy_managed_entity
    }
}
impl std::ops::DerefMut for ManagedPolicyVariable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_managed_entity
    }
}
