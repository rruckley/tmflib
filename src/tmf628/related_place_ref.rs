use serde::{Serialize, Deserialize};
use super::{PlaceRef};
use crate::common::extensible::Extensible;

///Related Place Reference
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedPlaceRef {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Place reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<PlaceRef>,
    ///Role of the related place.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl std::fmt::Display for RelatedPlaceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedPlaceRef {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedPlaceRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
