use serde::{Serialize, Deserialize};
use super::{
    AttachmentRefOrValue, CharacteristicSpecification, ConstraintRef,
    EntitySpecificationRelationship, RelatedParty, ServiceSpecificationRef,
    ServiceTestSpecRelationship, TargetEntitySchema, TestMeasureDefinition,
};
/**The service test specification describes the service test in terms of parameters to be configured and
measures to be taken.
Skipped properties: id,href,validFor*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceTestSpecificationUpdate {
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
    ///Description of a service test specification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Relationship to another specification
    #[serde(rename = "entitySpecRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_spec_relationship: Option<Vec<EntitySpecificationRelationship>>,
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
    ///The related service specification may relate to more than one service specification.
    #[serde(rename = "relatedServiceSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_service_specification: Option<Vec<ServiceSpecificationRef>>,
    ///A list of service test specifications related to this specification e.g. dependency, substitution
    #[serde(rename = "serviceTestSpecRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_test_spec_relationship: Option<Vec<ServiceTestSpecRelationship>>,
    ///List of characteristics that the entity can take
    #[serde(rename = "specCharacteristic")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec_characteristic: Option<Vec<CharacteristicSpecification>>,
    ///The reference object to the schema and type of target entity which is described by a specification
    #[serde(rename = "targetEntitySchema")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_entity_schema: Option<TargetEntitySchema>,
    ///A list of definitions for the measurements for the test defined by this specification
    #[serde(rename = "testMeasureDefinition")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_measure_definition: Option<Vec<TestMeasureDefinition>>,
    ///specification version
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for ServiceTestSpecificationUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
