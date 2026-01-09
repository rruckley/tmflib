use serde::{Serialize, Deserialize};
use super::{PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo, PolicyVariableFvo};
///Managed PolicyVariable
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyVariableFvo {
    /// Inlined fields from PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo
    #[serde(flatten)]
    pub policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo: PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo,
    ///A PolicyVariable is an entity for modeling different types of variables that can be used to form a PolicyCondition statement. It can be static or dynamic.
    #[serde(flatten)]
    pub policy_variable_fvo: PolicyVariableFvo,
}
impl std::fmt::Display for ManagedPolicyVariableFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyVariableFvo {
    type Target = PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo
    }
}
impl std::ops::DerefMut for ManagedPolicyVariableFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo
    }
}
