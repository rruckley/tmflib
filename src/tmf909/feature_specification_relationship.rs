use serde::{Serialize, Deserialize};
use crate::TimePeriod;
///Relationship between feature specifications
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureSpecificationRelationship {
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
    ///Unique identifier of the target feature specification.
    #[serde(rename = "featureId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<String>,
    ///This is the name of the target feature specification.
    pub name: String,
    ///Hyperlink reference to the parent specification containing the target feature
    #[serde(rename = "parentSpecificationHref")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_specification_href: Option<String>,
    ///Unique identifier of the parent specification containing the target feature
    #[serde(rename = "parentSpecificationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_specification_id: Option<String>,
    ///This is the type of the feature specification relationship.
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for FeatureSpecificationRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
