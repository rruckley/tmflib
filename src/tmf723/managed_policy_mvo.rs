use serde::{Serialize, Deserialize};
use super::{PolicyManagedEntityMvo, PolicyMvo};
///Managed Policy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyMvo {
    /// Inlined fields from PolicyManagedEntityMvo
    #[serde(flatten)]
    pub policy_managed_entity_mvo: PolicyManagedEntityMvo,
    ///Policy is a set of rules that are used to manage and control the state and state transitions of one or more managed objects.
    #[serde(flatten)]
    pub policy_mvo: PolicyMvo,
}
impl std::fmt::Display for ManagedPolicyMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyMvo {
    type Target = PolicyManagedEntityMvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_managed_entity_mvo
    }
}
impl std::ops::DerefMut for ManagedPolicyMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_managed_entity_mvo
    }
}
