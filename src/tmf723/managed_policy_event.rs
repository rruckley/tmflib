use serde::{Serialize, Deserialize};
use super::{PolicyEvent, PolicyManagedEntity};
///Managed PolicyEvent
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyEvent {
    ///A PolicyEvent is an occurrence of an important event or multiple events, and can be used to trigger the evaluation of a Policy
    #[serde(flatten)]
    pub policy_event: PolicyEvent,
    /// Inlined fields from PolicyManagedEntity
    #[serde(flatten)]
    pub policy_managed_entity: PolicyManagedEntity,
}
impl std::fmt::Display for ManagedPolicyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyEvent {
    type Target = PolicyEvent;
    fn deref(&self) -> &Self::Target {
        &self.policy_event
    }
}
impl std::ops::DerefMut for ManagedPolicyEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_event
    }
}
