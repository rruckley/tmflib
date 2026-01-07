use serde::{Serialize, Deserialize};
use super::UsageSpecification;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageSpecificationAttributeValueChangeEventPayload {
    ///A detailed description of a usage event that are of interest to the business and can have charges applied to it. It is comprised of characteristics, which define all attributes known for a particular type of usage.
    #[serde(rename = "usageSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_specification: Option<UsageSpecification>,
}
impl std::fmt::Display for UsageSpecificationAttributeValueChangeEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
