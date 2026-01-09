use serde::{Serialize, Deserialize};
use super::{PolicyConstraint, PolicyManagedEntity};
///Managed PolicyConstraint
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyConstraint {
    ///Any kind of condition that needs to be confirmed in order to proceed with next step
    #[serde(flatten)]
    pub policy_constraint: PolicyConstraint,
    ///Managed Entity attributes
    #[serde(flatten)]
    pub policy_managed_entity: PolicyManagedEntity,
}
impl std::fmt::Display for ManagedPolicyConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyConstraint {
    type Target = PolicyConstraint;
    fn deref(&self) -> &Self::Target {
        &self.policy_constraint
    }
}
impl std::ops::DerefMut for ManagedPolicyConstraint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_constraint
    }
}
