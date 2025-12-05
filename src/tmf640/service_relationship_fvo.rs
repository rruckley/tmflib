use serde::{Serialize, Deserialize};
use super::{CharacteristicFvo, ServiceRefOrValueFvo};
use crate::common::extensible::Extensible  ;

///This is a ServiceRelationship which may be represented either as a reference or as an inline value.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceRelationshipFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
    ///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the ServiceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceRefOrValueFvo>,
    #[serde(rename = "serviceRelationshipCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_relationship_characteristic: Vec<CharacteristicFvo>,
}
impl std::fmt::Display for ServiceRelationshipFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ServiceRelationshipFvo {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for ServiceRelationshipFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
