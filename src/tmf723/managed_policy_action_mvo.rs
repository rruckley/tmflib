use serde::{Serialize, Deserialize};
use super::{PolicyActionMvo, PolicyManagedEntityMvo};
///Managed PolicyAction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyActionMvo {
    /// Inlined fields from PolicyActionMvo
    #[serde(flatten)]
    pub policy_action_mvo: PolicyActionMvo,
    ///Inlined fields from PolicyManagedEntityMvo
    #[serde(flatten)]
    pub policy_managed_entity_mvo: PolicyManagedEntityMvo,
}
impl std::fmt::Display for ManagedPolicyActionMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyActionMvo {
    type Target = PolicyActionMvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_action_mvo
    }
}
impl std::ops::DerefMut for ManagedPolicyActionMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_action_mvo
    }
}
