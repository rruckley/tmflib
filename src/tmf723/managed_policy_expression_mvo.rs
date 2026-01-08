use serde::{Serialize, Deserialize};
use super::{PolicyExpressionMvo, PolicyManagedEntityMvo};
///Managed PolicyExpression
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyExpressionMvo {
    ///PolicyExpression is a constraint based on text expression and parsed by an Expression Language (SpEL, JS, Groovy, FEEL, ...)
    #[serde(flatten)]
    pub policy_expression_mvo: PolicyExpressionMvo,
    #[serde(flatten)]
    pub policy_managed_entity_mvo: PolicyManagedEntityMvo,
}
impl std::fmt::Display for ManagedPolicyExpressionMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyExpressionMvo {
    type Target = PolicyExpressionMvo;
    fn deref(&self) -> &Self::Target {
        &self.policy_expression_mvo
    }
}
impl std::ops::DerefMut for ManagedPolicyExpressionMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_expression_mvo
    }
}
