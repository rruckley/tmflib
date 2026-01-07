use serde::{Serialize, Deserialize};
use super::ProductOrderStateType;
use crate::common::entity::Entity;

///Base entity schema for use in TMForum Open-APIs. Property.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer360ProductOrderVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Used to categorize the order from a business perspective that can be useful for the OM system (e.g. "enterprise", "residential", ...)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    ///Date when the order was completed
    #[serde(rename = "completionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<crate::DateTime>,
    ///Date when the order was created
    #[serde(rename = "creationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<crate::DateTime>,
    ///Description of the product order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Expected delivery date amended by the provider
    #[serde(rename = "expectedCompletionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_completion_date: Option<crate::DateTime>,
    ///ID given by the consumer and only understandable by him (to facilitate his searches afterwards)
    #[serde(rename = "externalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    ///A way that can be used by consumers to prioritize orders in OM system (from 0 to 4 : 0 is the highest priority, and 4 the lowest)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    ///Possible values for the state of the order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ProductOrderStateType>,
}
impl std::fmt::Display for Customer360ProductOrderVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360ProductOrderVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360ProductOrderVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
