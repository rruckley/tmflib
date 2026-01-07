use super::Usage;
use serde::{Deserialize, Serialize};
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageAttributeValueChangeEventPayload {
    ///An occurrence of employing a Product, Service, or Resource for its intended purpose, which is of interest to the business and can have charges applied to it. It is comprised of characteristics, which represent attributes of usage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}
impl std::fmt::Display for UsageAttributeValueChangeEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
