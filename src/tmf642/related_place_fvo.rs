use serde::{Serialize, Deserialize};
use super::{Extensible, PlaceFvo};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedPlaceFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    #[serde(rename = "relatedPlace")]
    pub related_place: PlaceFvo,
    pub role: String,
}
impl std::fmt::Display for RelatedPlaceFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedPlaceFvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedPlaceFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
