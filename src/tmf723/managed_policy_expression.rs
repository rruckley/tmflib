use serde::{Serialize, Deserialize};
use super::{PolicyExpression, PolicyManagedEntity};
///Managed PolicyExpression
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedPolicyExpression {
    ///PolicyExpression is a constraint based on text expression and parsed by an Expression Language (SpEL, JS, Groovy, FEEL, ...)
    #[serde(flatten)]
    pub policy_expression: PolicyExpression,
    ///Managed Entity attributes
    #[serde(flatten)]
    pub policy_managed_entity: PolicyManagedEntity,
}
impl std::fmt::Display for ManagedPolicyExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagedPolicyExpression {
    type Target = PolicyExpression;
    fn deref(&self) -> &Self::Target {
        &self.policy_expression
    }
}
impl std::ops::DerefMut for ManagedPolicyExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_expression
    }
}
