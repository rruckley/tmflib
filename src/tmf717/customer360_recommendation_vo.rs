use serde::{Serialize, Deserialize};
use super::CategoryRef;
use crate::common::entity::Entity;
use crate::TimePeriod;

/// Customer360 Recommendation VO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer360RecommendationVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Category of the recommendation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<CategoryRef>,
    ///Description of the recommendation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Name of recommendation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The type of recommendation
    #[serde(rename = "recommendationType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommendation_type: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for Customer360RecommendationVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360RecommendationVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360RecommendationVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
