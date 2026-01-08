use serde::{Serialize, Deserialize};
use super::{PolicyManagedEntity, Reference};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyDomain {
    #[serde(flatten)]
    pub policy_managed_entity: PolicyManagedEntity,
    ///List of entities to which this PolicyDomain applies
    #[serde(rename = "scopedManagedEntity")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scoped_managed_entity: Vec<Reference>,
    ///List of child PolicyDomains. PolicyDomain can have many child Policy Domains, but one child PolicyDomain can only have one parent PolicyDomain
    #[serde(rename = "subDomainRef")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_domain_ref: Vec<Reference>,
}
impl std::fmt::Display for PolicyDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PolicyDomain {
    type Target = PolicyManagedEntity;
    fn deref(&self) -> &Self::Target {
        &self.policy_managed_entity
    }
}
impl std::ops::DerefMut for PolicyDomain {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.policy_managed_entity
    }
}
