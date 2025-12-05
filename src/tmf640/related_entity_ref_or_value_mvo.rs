use super::EntityRefOrValueMvo;
use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

///This is a related entity which may be represented either as a reference or as an inline value.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedEntityRefOrValueMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<EntityRefOrValueMvo>,
    ///Role of the related entity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl std::fmt::Display for RelatedEntityRefOrValueMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedEntityRefOrValueMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedEntityRefOrValueMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
