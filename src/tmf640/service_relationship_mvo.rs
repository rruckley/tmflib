use super::{CharacteristicMvo, ServiceRefOrValueMvo};
use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

///This is a relationship between two services.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceRelationshipMvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Type of relationship between services
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    ///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the ServiceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceRefOrValueMvo>,
    ///A list of characteristics that apply to this service relationship.
    #[serde(rename = "serviceRelationshipCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_relationship_characteristic: Vec<CharacteristicMvo>,
}
impl std::fmt::Display for ServiceRelationshipMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ServiceRelationshipMvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for ServiceRelationshipMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
