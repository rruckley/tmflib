use serde::{Serialize, Deserialize};
use super::{PolicyActionFvo, PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo};
///Managed PolicyAction
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyActionFvo {
    ///
    #[serde(flatten)]
    pub policy_action_fvo: PolicyActionFvo,
    /// Managed PolicyManagedEntity
    #[serde(flatten)]
    pub policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo: PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo,
}
impl std::fmt::Display for ManagedPolicyActionFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyActionFvo {
    type Target = PolicyActionFvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_action_fvo
    }
}
impl std::ops::DerefMut for ManagedPolicyActionFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_action_fvo
    }
}
