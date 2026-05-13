use super::MOD_PATH;
use super::{
    EntitySpecification, ResourceUsageSpecificationRef, ServiceSpecificationRef,
    ServiceUsageSpecRelationship, ServiceUsageSpecificationRef,
};
use crate::{HasId, IsAddressable};
use serde::{Deserialize, Serialize};
use tmflib_derive::HasId;

const CLASS_PATH: &str = "serviceUsageSpecification";

///Service Usage Specification represents a usage of a service specification, which may be part of a bundle and may have relationships to other service usage specifications and resource usage specifications. It is used to define the usage of a service specification in a specific context, e.g. for a specific customer or in a specific environment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, HasId)]
pub struct ServiceUsageSpecification {
    ///Unique identifier for this service usage specification
    pub id: Option<String>,
    ///Unique identifier for this service usage specification
    pub href: Option<String>,
    ///Base entity schema for use in `TMForum` Open-APIs. Property.
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
    ///Service specification reference. Service specification is a detailed description of a service that are of interest to the business. It is comprised of characteristics, which define all attributes known for a particular type of service.
    #[serde(rename = "serviceSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecificationRef>,
    ///A list of service usage specifications related to this specification, e.g. migration, substitution, dependency or exclusivity relationship
    #[serde(rename = "serviceUsageSpecRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_usage_spec_relationship: Vec<ServiceUsageSpecRelationship>,
}

impl IsAddressable for ServiceUsageSpecification {
    fn get_objects() -> Vec<&'static str> {
        super::get_objects()
    }
    fn get_version() -> &'static str {
        super::get_version()
    }
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
