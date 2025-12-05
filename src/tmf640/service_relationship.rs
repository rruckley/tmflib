use serde::{Serialize, Deserialize};
use super::{Characteristic, Extensible, ServiceRefOrValue};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceRelationship {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    ///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the ServiceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<ServiceRefOrValue>,
    #[serde(rename = "serviceRelationshipCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_relationship_characteristic: Vec<Characteristic>,
}
impl std::fmt::Display for ServiceRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ServiceRelationship {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for ServiceRelationship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
