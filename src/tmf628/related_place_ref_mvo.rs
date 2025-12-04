use super::PlaceRefMvo;
use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

///Related Place Reference MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedPlaceRefMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Place reference.
    pub place: PlaceRefMvo,
    ///Role of the related place.
    pub role: String,
}
impl std::fmt::Display for RelatedPlaceRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedPlaceRefMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedPlaceRefMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
