use super::{Characteristic, ResourceRefOrValue};
use crate::common::extensible::Extensible;
use serde::{Deserialize, Serialize};

///Resource Relationship
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceRelationship {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Type of the resource relationship, such as [bundled] if the resource is a bundle and you want to describe the bundled resources inside this bundle; [reliesOn] if the resource needs another already owned resource to rely on (eg: an option on an already owned mobile access resource) [targets] or [isTargeted] (depending on the way of expressing the link) for any other kind of links that may be useful
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    ///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Resource entity and not the ResourceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceRefOrValue>,
    ///A list of characteristics that apply to the resource relationship.
    #[serde(rename = "resourceRelationshipCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_relationship_characteristic: Vec<Characteristic>,
}
impl std::fmt::Display for ResourceRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ResourceRelationship {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for ResourceRelationship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
