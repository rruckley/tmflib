use serde::{Serialize, Deserialize};
use super::{PolicyConstraintFvo, PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo};
///Managed PolicyConstraint
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyConstraintFvo {
    ///Any kind of condition that needs to be confirmed in order to proceed with next step
    #[serde(flatten)]
    pub policy_constraint_fvo: PolicyConstraintFvo,
    #[serde(flatten)]
    pub policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo: PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo,
}
impl std::fmt::Display for ManagedPolicyConstraintFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyConstraintFvo {
    type Target = PolicyConstraintFvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_constraint_fvo
    }
}
impl std::ops::DerefMut for ManagedPolicyConstraintFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_constraint_fvo
    }
}
