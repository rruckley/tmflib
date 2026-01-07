use serde::{Serialize, Deserialize};
use super::{ServiceCategoryRef, ServiceSpecificationRef};
use crate::TimePeriod;

/**ServiceCandidate is an entity that makes a service specification available to a catalog. A
ServiceCandidate and its associated service specification may be published - made visible - in any number of service catalogs, or in none. One service specification can be composed of other service specifications.
Skipped properties: id,href*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceCandidateCreate {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///List of categories for this candidate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<ServiceCategoryRef>>,
    ///Description of this REST resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Date and time of the last update of this REST resource
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Used to indicate the current lifecycle status of the service candidate.
    #[serde(rename = "lifecycleStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    ///Name given to this REST resource
    pub name: String,
    ///Service specification reference: ServiceSpecification(s) required to realize a ProductSpecification.
    #[serde(rename = "serviceSpecification")]
    pub service_specification: ServiceSpecificationRef,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///the version of service candidate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ServiceCandidateCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
