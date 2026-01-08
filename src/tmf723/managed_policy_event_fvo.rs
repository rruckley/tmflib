use serde::{Serialize, Deserialize};
use super::{PolicyEventFvo, PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo};
///Managed PolicyEvent
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyEventFvo {
    ///A PolicyEvent is an occurrence of an important event or multiple events, and can be used to trigger the evaluation of a Policy
    #[serde(flatten)]
    pub policy_event_fvo: PolicyEventFvo,
    #[serde(flatten)]
    pub policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo: PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo,
}
impl std::fmt::Display for ManagedPolicyEventFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyEventFvo {
    type Target = PolicyEventFvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_event_fvo
    }
}
impl std::ops::DerefMut for ManagedPolicyEventFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_event_fvo
    }
}
