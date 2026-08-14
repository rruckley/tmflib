use super::{ServiceCategoryRef, ServiceSpecificationRef};
use crate::{HasDescription, HasId, IsAddressable, TimePeriod};
use serde::{Deserialize, Serialize};
use tmflib_derive::{HasDescription, HasId};

// URL Path components
use super::TMF_MODULE;
use crate::TMF_VERSION;
/// Path to module
pub const CLASS_PATH: &str = "serviceCandidate";

/// `ServiceCandidate` service specification in a catalog
///
/// A `ServiceCandidate` represents a service specification that is offered to customers. It describes the characteristics of a service that can be offered as part of a product offering. It may also include information about the service's lifecycle status, valid time period, and other attributes. A `ServiceCandidate` is associated with a `ServiceSpecification`, which provides more detailed information about the service's features and requirements.
/// A `ServiceCandidate` and its associated service specification may be published - made visible - in any number of service catalogs, or in none. One service specification can be composed of other service specifications.*/
#[derive(Debug, Clone, Serialize, HasId, HasDescription, Deserialize, Default)]
pub struct ServiceCandidate {
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
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Date and time of the last update of this REST resource
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Used to indicate the current lifecycle status of the service candidate.
    #[serde(rename = "lifecycleStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    ///Name given to this REST resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Service specification reference: ServiceSpecification(s) required to realize a `ProductSpecification`.
    #[serde(rename = "serviceSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecificationRef>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///the version of service candidate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl IsAddressable for ServiceCandidate {
    fn get_objects() -> Vec<&'static str> {
        super::get_objects()
    }
}
impl std::fmt::Display for ServiceCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
