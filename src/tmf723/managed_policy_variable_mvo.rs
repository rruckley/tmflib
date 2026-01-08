use serde::{Serialize, Deserialize};
use super::{PolicyManagedEntityMvo, PolicyVariableMvo};
///Managed PolicyVariable
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyVariableMvo {
    #[serde(flatten)]
    pub policy_managed_entity_mvo: PolicyManagedEntityMvo,
    ///A PolicyVariable is an entity for modeling different types of variables that can be used to form a PolicyCondition statement. It can be static or dynamic.
    #[serde(flatten)]
    pub policy_variable_mvo: PolicyVariableMvo,
}
impl std::fmt::Display for ManagedPolicyVariableMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyVariableMvo {
    type Target = PolicyManagedEntityMvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_managed_entity_mvo
    }
}
impl std::ops::DerefMut for ManagedPolicyVariableMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_managed_entity_mvo
    }
}
