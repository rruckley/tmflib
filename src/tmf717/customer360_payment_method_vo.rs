use serde::{Serialize, Deserialize};
use crate::common::entity::Entity;
use crate::TimePeriod;

///Base entity schema for use in TMForum Open-APIs. Property.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer360PaymentMethodVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///The contents of the payment method
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///If the method is the preferred one by the owner. Typically used when querying for the payment methods of a specific customer or account
    #[serde(rename = "isPreferred")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_preferred: Option<bool>,
    ///Friendly name assigned to the payment method
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Status of the payment method
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for Customer360PaymentMethodVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360PaymentMethodVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360PaymentMethodVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
