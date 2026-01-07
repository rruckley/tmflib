use serde::{Serialize, Deserialize};
use super::PromotionCriteriaMvo;
///Set of group criteria to promotion. Fulfilling these criteria, parties will receice benefits
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromotionCriteriaGroupMvo {
    ///List of criteria included in this group
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Vec<PromotionCriteriaMvo>>,
    ///Logical relation followed by all criteria.
    #[serde(rename = "criteriaLogicalRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria_logical_relationship: Option<String>,
    ///Name of the group to be easily identified
    #[serde(rename = "groupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}
impl std::fmt::Display for PromotionCriteriaGroupMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
