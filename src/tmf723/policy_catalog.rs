use serde::{Serialize, Deserialize};
use super::{
    ManagedPolicy, ManagedPolicyAction, ManagedPolicyCondition, ManagedPolicyConstraint,
    ManagedPolicyEvent, ManagedPolicyExpression, ManagedPolicyOperator,
    ManagedPolicyVariable, PolicyManagedEntity, Reference,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyCatalog {
    #[serde(flatten)]
    pub policy_managed_entity: PolicyManagedEntity,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<ManagedPolicy>,
    #[serde(rename = "policyAction")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_action: Vec<ManagedPolicyAction>,
    #[serde(rename = "policyCondition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_condition: Vec<ManagedPolicyCondition>,
    #[serde(rename = "policyConstraint")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_constraint: Vec<ManagedPolicyConstraint>,
    #[serde(rename = "policyDomainFilter")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_domain_filter: Vec<Reference>,
    #[serde(rename = "policyEvent")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_event: Vec<ManagedPolicyEvent>,
    #[serde(rename = "policyExpression")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_expression: Vec<ManagedPolicyExpression>,
    #[serde(rename = "policyOperator")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_operator: Vec<ManagedPolicyOperator>,
    #[serde(rename = "policyVariable")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_variable: Vec<ManagedPolicyVariable>,
}
impl std::fmt::Display for PolicyCatalog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PolicyCatalog {
    type Target = PolicyManagedEntity;
    fn deref(&self) -> &Self::Target {
        &self.policy_managed_entity
    }
}
impl std::ops::DerefMut for PolicyCatalog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_managed_entity
    }
}
