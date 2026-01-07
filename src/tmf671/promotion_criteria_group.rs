use super::PromotionCriteria;
use serde::{Deserialize, Serialize};
///Set of group criteria to promotion. Fulfilling these criteria, parties will receice benefits
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromotionCriteriaGroup {
    ///The base type for use in polymorphic collections
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A link to the schema describing a resource (for type extension).
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///The class type of the actual resource (for type extension).
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///List of criteria included in this group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Vec<PromotionCriteria>>,
    ///Logical relation followed by all criteria.
    #[serde(rename = "criteriaLogicalRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria_logical_relationship: Option<String>,
    ///Name of the group to be easily identified
    #[serde(rename = "groupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    ///Unique Identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl std::fmt::Display for PromotionCriteriaGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
