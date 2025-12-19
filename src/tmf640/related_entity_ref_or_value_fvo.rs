use super::EntityRefOrValueFvo;
use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

///This is a related entity which may be represented either as a reference or as a value object.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedEntityRefOrValueFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///
    pub entity: EntityRefOrValueFvo,
    ///Role of the related entity
    pub role: String,
}
impl std::fmt::Display for RelatedEntityRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedEntityRefOrValueFvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedEntityRefOrValueFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
