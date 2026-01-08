use serde::{Serialize, Deserialize};
use super::ServiceCategory;
///The event data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceCategoryChangeEventPayload {
    ///The (service) category resource is used to group service candidates in logical containers. Categories can contain other categories.
    #[serde(rename = "serviceCategory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_category: Option<ServiceCategory>,
}
impl std::fmt::Display for ServiceCategoryChangeEventPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
