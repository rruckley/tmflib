use serde::{Serialize, Deserialize};
use super::{
    AttachmentRefOrValue, CharacteristicSpecification, ConstraintRef,
    EntitySpecificationRelationship, RelatedParty, ResourceSpecificationRef,
    ServiceFeatureSpecification, ServiceLevelSpecificationRef, ServiceSpecRelationship,
    TargetEntitySchema,
};
use crate::TimePeriod;

/// ServiceSpecification is a class that offers characteristics to describe a type of service.
/// Functionally, it acts as a template by which Services may be instantiated. By sharing the same specification, these services would therefore share the same set of characteristics.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Attachments that may be of relevance to this specification, such as picture, document, media
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
    ///This is a list of constraint references applied to this specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraint: Option<Vec<ConstraintRef>>,
    ///Description of the specification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Relationship to another specification
    #[serde(rename = "entitySpecRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_spec_relationship: Option<Vec<EntitySpecificationRelationship>>,
    ///A list of Features for this specification.
    #[serde(rename = "featureSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_specification: Option<Vec<ServiceFeatureSpecification>>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    ///A list of resource specification references (ResourceSpecificationRef [*]). The ResourceSpecification is required for a service specification with type ResourceFacingServiceSpecification (RFSS).
    #[serde(rename = "resourceSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<Vec<ResourceSpecificationRef>>,
    ///A list of service level specifications related to this service specification, and which will need to be satisifiable for corresponding service instances; e.g. Gold, Platinum
    #[serde(rename = "serviceLevelSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_level_specification: Option<Vec<ServiceLevelSpecificationRef>>,
    ///A list of service specifications related to this specification, e.g. migration, substitution, dependency or exclusivity relationship
    #[serde(rename = "serviceSpecRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_spec_relationship: Option<Vec<ServiceSpecRelationship>>,
    ///List of characteristics that the entity can take
    #[serde(rename = "specCharacteristic")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec_characteristic: Option<Vec<CharacteristicSpecification>>,
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
impl std::fmt::Display for ServiceSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
