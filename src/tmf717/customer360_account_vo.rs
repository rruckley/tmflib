use serde::{Serialize, Deserialize};
use super::Entity;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer360AccountVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///A categorization of an account, such as individual, joint, and so forth, whose instances share some of the same characteristics. Note: for flexibility we use a String here but an implementation may use an enumeration with a limited list of valid values.
    #[serde(rename = "accountType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    ///Detailed description of the party account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Date of last modification of the account
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Name of the account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Contains the lifecycle state such as: Active, Closed, Suspended and so on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for Customer360AccountVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360AccountVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360AccountVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
