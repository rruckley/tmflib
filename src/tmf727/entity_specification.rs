use super::{
    AttachmentRefOrValue,
    CharacteristicSpecification,
    ConstraintRef,
    EntitySpecificationRelationship,
    RelatedPartyRefOrPartyRoleRef,
    TargetEntitySchema,
    // TimePeriod,
};
use crate::common::entity::Entity;
use serde::{Deserialize, Serialize};
// use crate::common::addressable::Addressable;
use crate::TimePeriod;

///`EntitySpecification` represents a detailed description of an entity, which may include the version of the entity. It is used to define the specification of an entity in a specific context, e.g. for a specific customer or in a specific environment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntitySpecification {
    ///Base entity schema for use in `TMForum` Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Attachments that may be of relevance to this specification, such as picture, document, media
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachment: Vec<AttachmentRefOrValue>,
    ///This is a list of constraint references applied to this specification
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub constraint: Vec<ConstraintRef>,
    ///Description of the specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Relationship to another specification
    #[serde(rename = "entitySpecRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entity_spec_relationship: Vec<EntitySpecificationRelationship>,
    ///isBundle determines whether specification represents a single specification (false), or a bundle of specifications (true).
    #[serde(rename = "isBundle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    ///Date and time of the last update of the specification
    #[serde(rename = "lastUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    ///Used to indicate the current lifecycle status of this catalog item
    #[serde(rename = "lifecycleStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_status: Option<String>,
    ///Name given to the specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Parties who manage or otherwise have an interest in this specification
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyRefOrPartyRoleRef>,
    ///List of characteristics that the entity can take
    #[serde(rename = "specCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spec_characteristic: Vec<CharacteristicSpecification>,
    ///The reference object to the schema and type of target entity which is described by a specification
    #[serde(rename = "targetEntitySchema")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_entity_schema: Option<TargetEntitySchema>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///specification version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for EntitySpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for EntitySpecification {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for EntitySpecification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
