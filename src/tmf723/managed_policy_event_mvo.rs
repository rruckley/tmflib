use serde::{Serialize, Deserialize};
use super::{PolicyEventMvo, PolicyManagedEntityMvo};
///Managed PolicyEvent
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyEventMvo {
    ///A PolicyEvent is an occurrence of an important event or multiple events, and can be used to trigger the evaluation of a Policy
    #[serde(flatten)]
    pub policy_event_mvo: PolicyEventMvo,
    ///Managed Entity MVO attributes
    #[serde(flatten)]
    pub policy_managed_entity_mvo: PolicyManagedEntityMvo,
}
impl std::fmt::Display for ManagedPolicyEventMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyEventMvo {
    type Target = PolicyEventMvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_event_mvo
    }
}
impl std::ops::DerefMut for ManagedPolicyEventMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_event_mvo
    }
}
