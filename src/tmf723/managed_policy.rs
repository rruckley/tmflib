use serde::{Serialize, Deserialize};
use super::{Policy, PolicyManagedEntity};
///Managed Policy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicy {
    ///Policy is a set of rules that are used to manage and control the state and state transitions of one or more managed objects.
    #[serde(flatten)]
    pub policy: Policy,
    #[serde(flatten)]
    pub policy_managed_entity: PolicyManagedEntity,
}
impl std::fmt::Display for ManagedPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicy {
    type Target = Policy;
    fn deref(&self) -> &Self::Target {
        &self.policy
    }
}
impl std::ops::DerefMut for ManagedPolicy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy
    }
}
