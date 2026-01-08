use serde::{Serialize, Deserialize};
use super::{PolicyConditionMvo, PolicyManagedEntityMvo};
///Managed PolicyCondition
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyConditionMvo {
    ///A PolicyCondition clause is an aggregation of individual PolicyConditions, and is treated as an atomic object that is aggregated by a PolicyRule. It is represented as a Boolean expression, and defines the necessary state and/or prerequisites that define whether the actions aggregated by that same PolicyRule should be performed. If PolicyCondition is successfully resolved then it must hold value TRUE or FALSE. Non successfull resolution does not contain any boolean value
    #[serde(flatten)]
    pub policy_condition_mvo: PolicyConditionMvo,
    #[serde(flatten)]
    pub policy_managed_entity_mvo: PolicyManagedEntityMvo,
}
impl std::fmt::Display for ManagedPolicyConditionMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyConditionMvo {
    type Target = PolicyConditionMvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_condition_mvo
    }
}
impl std::ops::DerefMut for ManagedPolicyConditionMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_condition_mvo
    }
}
