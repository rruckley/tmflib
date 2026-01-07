use super::PromotionPattern;
use crate::TimePeriod;
use serde::{Deserialize, Serialize};

///Promotion Resource is used to provide additional discounts, vouchers, bonuses, or gifts to customers who meet predefined criteria. By using promotions, the enterprise can attract users and encourage more consumption, especially continuous purchases. Normally, promotions are not considered a specific type of product or product offering. They are often applied when customers purchase product offerings that exceed a certain price or amount limit.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Promotion {
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
    ///Detailed pattern of the promotion.The pattern decides the conditions of promotion and the benefit of the promotion to be given to the eligible customer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Vec<PromotionPattern>>,
    ///Type of promotion.The basic type is Award/Discount/Reduction. More types can be extended in future.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for Promotion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
