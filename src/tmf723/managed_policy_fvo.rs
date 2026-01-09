use serde::{Serialize, Deserialize};
use super::{PolicyFvo, PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo};
///Managed Policy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyFvo {
    ///Managed Entity FVO attributes
    #[serde(flatten)]
    pub policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo: PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo,
    ///Policy is a set of rules that are used to manage and control the state and state transitions of one or more managed objects.
    #[serde(flatten)]
    pub policy_fvo: PolicyFvo,
}
impl std::fmt::Display for ManagedPolicyFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyFvo {
    type Target = PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo
    }
}
impl std::ops::DerefMut for ManagedPolicyFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo
    }
}
