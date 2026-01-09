use serde::{Serialize, Deserialize};
use super::{PolicyExpressionFvoFvoFvo, PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo};
///Managed PolicyExpression
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyExpressionFvo {
    ///PolicyExpression is a constraint based on text expression and parsed by an Expression Language (SpEL, JS, Groovy, FEEL, ...)
    #[serde(flatten)]
    pub policy_expression_fvo_fvo_fvo: PolicyExpressionFvoFvoFvo,
    ///Managed Entity FVO attributes
    #[serde(flatten)]
    pub policy_managed_entity_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo_fvo: PolicyManagedEntityFvoFvoFvoFvoFvoFvoFvoFvoFvo,
}
impl std::fmt::Display for ManagedPolicyExpressionFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyExpressionFvo {
    type Target = PolicyExpressionFvoFvoFvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_expression_fvo_fvo_fvo
    }
}
impl std::ops::DerefMut for ManagedPolicyExpressionFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_expression_fvo_fvo_fvo
    }
}
