use serde::{Serialize, Deserialize};
// use super::{Addressable, Extensible};
use crate::common::addressable::Addressable;

/// Entity Reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityRefXXX {
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///Base schema for addressable entities
    #[serde(flatten)]
    pub addressable: Addressable,
    ///Base Extensible schema for use in TMForumX,
    ///The URI of the referred entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///The identifier of the referred entity.
    pub id: String,
    ///Name of the referred entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for EntityRefXXX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for EntityRefXXX {
    type Target = Addressable;
    fn deref(&self) -> &Self::Target {
        &self.addressable
    }
}
impl std::ops::DerefMut for EntityRefXXX {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.addressable
    }
}
