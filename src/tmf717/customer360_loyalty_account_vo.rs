use serde::{Serialize, Deserialize};
use super::Entity;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer360LoyaltyAccountVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Balances linked to the account
    #[serde(rename = "accountBalance")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<serde_json::Value>,
    ///Name of the account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Contains the lifecycle state such as: Active, Closed, Suspended and so on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for Customer360LoyaltyAccountVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360LoyaltyAccountVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360LoyaltyAccountVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
