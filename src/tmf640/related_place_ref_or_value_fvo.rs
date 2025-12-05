use serde::{Serialize, Deserialize};
use super::{PlaceRefOrValueFvo};
use crate::common::extensible::Extensible  ;

///A related place, with a role.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelatedPlaceRefOrValueFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Place entity and not the PlaceRefOrValue class itself
    pub place: PlaceRefOrValueFvo,
    pub role: String,
}
impl std::fmt::Display for RelatedPlaceRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for RelatedPlaceRefOrValueFvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for RelatedPlaceRefOrValueFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
