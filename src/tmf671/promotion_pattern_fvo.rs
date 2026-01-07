use serde::{Serialize, Deserialize};
use super::{PromotionActionFvo, PromotionCriteriaGroupFvo};
use crate::TimePeriod;
///Detailed pattern of the promotion.The pattern decides the conditions of promotion and the benefit of the promotion to be given to the eligible customer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromotionPatternFvo {
    ///Action of the promotion. When the customer meets the conditions in the promotion pattern, the customer can be given the benefits in the action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<PromotionActionFvo>>,
    ///Set of group criteria to promotion. Fulfilling these criteria, parties will receice benefits
    #[serde(rename = "criteriaGroup")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria_group: Option<Vec<PromotionCriteriaGroupFvo>>,
    ///Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Priority. Smaller number means high.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    ///AND/OR ,The logical relation type amongst the promotion criteria group.
    #[serde(rename = "relationTypeAmongGroup")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation_type_among_group: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for PromotionPatternFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
