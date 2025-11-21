use serde::{Serialize, Deserialize};
use super::{CharacteristicFvo, ExtensibleFvo, ResourceRefOrValueFvo};

///Resource Relationship FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceRelationshipFvo {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible_fvo: ExtensibleFvo,
    ///Type of the resource relationship, such as [bundled] if the resource is a bundle and you want to describe the bundled resources inside this bundle; [reliesOn] if the resource needs another already owned resource to rely on (eg: an option on an already owned mobile access resource) [targets] or [isTargeted] (depending on the way of expressing the link) for any other kind of links that may be useful
    #[serde(rename = "relationshipType")]
    pub relationship_type: String,
    ///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Resource entity and not the ResourceRefOrValue class itself
    pub resource: ResourceRefOrValueFvo,
    ///List of characteristics
    #[serde(rename = "resourceRelationshipCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_relationship_characteristic: Vec<CharacteristicFvo>,
}
impl std::fmt::Display for ResourceRelationshipFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ResourceRelationshipFvo {
    type Target = ExtensibleFvo;
    fn deref(&self) -> &Self::Target {
        &self.extensible_fvo
    }
}
impl std::ops::DerefMut for ResourceRelationshipFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible_fvo
    }
}
