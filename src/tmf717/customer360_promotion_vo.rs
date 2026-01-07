use serde::{Serialize, Deserialize};
use super::{Entity, TimePeriod};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer360PromotionVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Description of Promotion
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Latest update date of Promotion
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Status of Promotion, including draft/Test/WaitForApproval/Release/Suspend/Retirement.
    #[serde(rename = "lifecycleStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    ///Name of Promotion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Type of promotion.The basic type is Award/Discount/Reduction. More types can be extended in future.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for Customer360PromotionVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360PromotionVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360PromotionVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
