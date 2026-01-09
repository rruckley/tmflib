use serde::{Serialize, Deserialize};
use super::{PolicyConstraintMvo, PolicyManagedEntityMvo};
///Managed PolicyConstraint
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyConstraintMvo {
    ///Any kind of condition that needs to be confirmed in order to proceed with next step
    #[serde(flatten)]
    pub policy_constraint_mvo: PolicyConstraintMvo,
    ///Managed Entity MVO attributes
    #[serde(flatten)]
    pub policy_managed_entity_mvo: PolicyManagedEntityMvo,
}
impl std::fmt::Display for ManagedPolicyConstraintMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyConstraintMvo {
    type Target = PolicyConstraintMvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_constraint_mvo
    }
}
impl std::ops::DerefMut for ManagedPolicyConstraintMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_constraint_mvo
    }
}
