use serde::{Serialize, Deserialize};
use super::{Extensible, Place};

/// RelatedPlaceFvo defines a related place for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedPlace {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    /// Reference to the related place
    #[serde(rename = "relatedPlace")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_place: Option<Place>,
    /// Role played by the related place
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl std::fmt::Display for RelatedPlace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedPlace {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedPlace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
