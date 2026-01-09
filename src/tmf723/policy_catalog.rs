use serde::{Serialize, Deserialize};
use super::{
    ManagedPolicy, ManagedPolicyAction, ManagedPolicyCondition, ManagedPolicyConstraint,
    ManagedPolicyEvent, ManagedPolicyExpression, ManagedPolicyOperator,
    ManagedPolicyVariable, PolicyManagedEntity, Reference,
};

/// PolicyCatalog represents a collection of managed policies and related entities.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyCatalog {
    /// Inlined fields from PolicyManagedEntity
    #[serde(flatten)]
    pub policy_managed_entity: PolicyManagedEntity,
    /// Collections of various managed policy components
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy: Vec<ManagedPolicy>,
    /// Collections of policy actions
    #[serde(rename = "policyAction")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_action: Vec<ManagedPolicyAction>,
    /// Collections of policy conditions
    #[serde(rename = "policyCondition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_condition: Vec<ManagedPolicyCondition>,
    /// Collections of policy constraints
    #[serde(rename = "policyConstraint")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_constraint: Vec<ManagedPolicyConstraint>,
    /// Collections of policy domain filters
    #[serde(rename = "policyDomainFilter")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_domain_filter: Vec<Reference>,
    /// Collections of policy events
    #[serde(rename = "policyEvent")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_event: Vec<ManagedPolicyEvent>,
    /// Collections of policy expressions
    #[serde(rename = "policyExpression")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_expression: Vec<ManagedPolicyExpression>,
    /// Collections of policy operators
    #[serde(rename = "policyOperator")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub policy_operator: Vec<ManagedPolicyOperator>,
    /// Collections of policy variables
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
