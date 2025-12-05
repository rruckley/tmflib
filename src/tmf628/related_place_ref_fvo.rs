use super::{PlaceRefFvo};
use serde::{Deserialize, Serialize};
use crate::common::extensible::ExtensibleFvo;

///Related Place Reference FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedPlaceRefFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///Place reference.
    pub place: PlaceRefFvo,
    ///Role of the related place.
    pub role: String,
}
impl std::fmt::Display for RelatedPlaceRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedPlaceRefFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for RelatedPlaceRefFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
