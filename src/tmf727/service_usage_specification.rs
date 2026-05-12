use serde::{Serialize, Deserialize};
use super::{
    EntitySpecification, ResourceUsageSpecificationRef, ServiceSpecificationRef,
    ServiceUsageSpecRelationship, ServiceUsageSpecificationRef,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceUsageSpecification {
    #[serde(flatten)]
    pub entity_specification: EntitySpecification,
    ///Relationship to Service specification in bundle
    #[serde(rename = "bundledServiceUsageSpecification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bundled_service_usage_specification: Vec<ServiceUsageSpecificationRef>,
    ///Relationship to resource usage specification
    #[serde(rename = "resourceUsageSpecification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_usage_specification: Vec<ResourceUsageSpecificationRef>,
    #[serde(rename = "serviceSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecificationRef>,
    ///A list of service usage specifications related to this specification, e.g. migration, substitution, dependency or exclusivity relationship
    #[serde(rename = "serviceUsageSpecRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_usage_spec_relationship: Vec<ServiceUsageSpecRelationship>,
}
impl std::fmt::Display for ServiceUsageSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ServiceUsageSpecification {
    type Target = EntitySpecification;
    fn deref(&self) -> &Self::Target {
        &self.entity_specification
    }
}
impl std::ops::DerefMut for ServiceUsageSpecification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity_specification
    }
}
