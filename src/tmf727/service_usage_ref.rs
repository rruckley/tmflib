use serde::{Serialize, Deserialize};
use crate::common::entity::Entity;

///Reference to a service usage, which is a specialization of an entity reference with a name and referredType
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceUsageRef {
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///The name of the usage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for ServiceUsageRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ServiceUsageRef {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for ServiceUsageRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
