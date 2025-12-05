use serde::{Serialize, Deserialize};
use super::{PlaceRefOrValueMvo};
use crate::common::extensible::Extensible  ;

///A related place, with a role.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedPlaceRefOrValueMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Place entity and not the PlaceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<PlaceRefOrValueMvo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl std::fmt::Display for RelatedPlaceRefOrValueMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedPlaceRefOrValueMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedPlaceRefOrValueMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
