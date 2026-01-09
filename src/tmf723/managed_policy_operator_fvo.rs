use serde::{Serialize, Deserialize};
use super::{PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo, PolicyOperatorFvo};
///Managed PolicyOperator
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyOperatorFvo {
    ///Managed Entity FVO attributes
    #[serde(flatten)]
    pub policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo: PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo,
    ///A PolicyOperator defines a logical operator (AND, OR, NOT, ...) that is used to combine multiple PolicyConditions or PolicyExpressions together into a single logical unit that can be evaluated as TRUE or FALSE
    #[serde(flatten)]
    pub policy_operator_fvo: PolicyOperatorFvo,
}
impl std::fmt::Display for ManagedPolicyOperatorFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyOperatorFvo {
    type Target = PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo
    }
}
impl std::ops::DerefMut for ManagedPolicyOperatorFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo
    }
}
