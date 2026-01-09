use serde::{Serialize, Deserialize};
use super::{PolicyConditionFvo, PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo};
///Managed PolicyCondition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyConditionFvo {
    ///A PolicyCondition clause is an aggregation of individual PolicyConditions, and is treated as an atomic object that is aggregated by a PolicyRule. It is represented as a Boolean expression, and defines the necessary state and/or prerequisites that define whether the actions aggregated by that same PolicyRule should be performed. If PolicyCondition is successfully resolved then it must hold value TRUE or FALSE. Non successfull resolution does not contain any boolean value
    #[serde(flatten)]
    pub policy_condition_fvo: PolicyConditionFvo,
    ///Managed Entity FVO attributes
    #[serde(flatten)]
    pub policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo: PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo,
}
impl std::fmt::Display for ManagedPolicyConditionFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyConditionFvo {
    type Target = PolicyConditionFvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_condition_fvo
    }
}
impl std::ops::DerefMut for ManagedPolicyConditionFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_condition_fvo
    }
}
