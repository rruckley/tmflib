use serde::{Serialize, Deserialize};
use super::{ServiceCandidateRef, ServiceCategoryRef};
use crate::TimePeriod;

/// The (service) category resource is used to group service candidates in logical containers. Categories can contain other categories.
/// Skipped properties: id,href
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceCategoryCreate {
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
    ///List of child categories in the tree for in this category
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<ServiceCategoryRef>>,
    ///Description of the category
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///If true, this Boolean indicates that the category is a root of categories
    #[serde(rename = "isRoot")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_root: Option<bool>,
    ///Date and time of the last update
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Used to indicate the current lifecycle status
    #[serde(rename = "lifecycleStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    ///Name of the category
    pub name: String,
    ///Unique identifier of the parent category
    #[serde(rename = "parentId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    ///List of service candidates associated with this category
    #[serde(rename = "serviceCandidate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_candidate: Option<Vec<ServiceCandidateRef>>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///ServiceCategory version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ServiceCategoryCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
