use super::{Addressable, Extensible};
use serde::{Deserialize, Serialize};

///EntityRefFvo is a generic reference to an entity that is identified by at least an id. The referred entity may be extended to capture additional attributes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityRefFvo {
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///Base schema for addressable entities
    #[serde(flatten)]
    pub addressable: Addressable,
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///The identifier of the referred entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Name of the referred entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for EntityRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for EntityRefFvo {
    type Target = Addressable;
    fn deref(&self) -> &Self::Target {
        &self.addressable
    }
}
impl std::ops::DerefMut for EntityRefFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.addressable
    }
}
