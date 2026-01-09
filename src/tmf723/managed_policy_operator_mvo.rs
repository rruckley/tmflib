use serde::{Serialize, Deserialize};
use super::{PolicyManagedEntityMvo, PolicyOperatorMvo};
///Managed PolicyOperator
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyOperatorMvo {
    /// Inlined fields from PolicyManagedEntityMvo
    #[serde(flatten)]
    pub policy_managed_entity_mvo: PolicyManagedEntityMvo,
    ///A PolicyOperator is an entity that defines the logical operations that can be used to combine multiple PolicyConditions within a PolicyExpression.
    #[serde(flatten)]
    pub policy_operator_mvo: PolicyOperatorMvo,
}
impl std::fmt::Display for ManagedPolicyOperatorMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyOperatorMvo {
    type Target = PolicyManagedEntityMvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_managed_entity_mvo
    }
}
impl std::ops::DerefMut for ManagedPolicyOperatorMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_managed_entity_mvo
    }
}
