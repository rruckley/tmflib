use serde::{Serialize, Deserialize};
use super::{PolicyManagedEntity, PolicyOperator};
///Managed PolicyOperator
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyOperator {
    #[serde(flatten)]
    pub policy_managed_entity: PolicyManagedEntity,
    #[serde(flatten)]
    pub policy_operator: PolicyOperator,
}
impl std::fmt::Display for ManagedPolicyOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyOperator {
    type Target = PolicyManagedEntity;
    fn deref(&self) -> &Self::Target {
        &self.policy_managed_entity
    }
}
impl std::ops::DerefMut for ManagedPolicyOperator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_managed_entity
    }
}
