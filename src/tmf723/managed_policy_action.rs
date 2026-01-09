use serde::{Serialize, Deserialize};
use super::{PolicyAction, PolicyManagedEntity};
///Managed PolicyAction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyAction {
    /// PolicyAction
    #[serde(flatten)]
    pub policy_action: PolicyAction,
    /// Managed PolicyManagedEntity
    #[serde(flatten)]
    pub policy_managed_entity: PolicyManagedEntity,
}
impl std::fmt::Display for ManagedPolicyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyAction {
    type Target = PolicyAction;
    fn deref(&self) -> &Self::Target {
        &self.policy_action
    }
}
impl std::ops::DerefMut for ManagedPolicyAction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_action
    }
}
