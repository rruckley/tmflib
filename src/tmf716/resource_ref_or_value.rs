use super::{
    AttachmentRefOrValue, Characteristic, Feature, RelatedParty, RelatedPlaceRefOrValue,
    ResourceAdministrativeStateType, ResourceOperationalStateType, ResourceRelationship,
    ResourceSpecificationRef, ResourceStatusType, ResourceUsageStateType,
};
use crate::common::note::Note;
use serde::{Deserialize, Serialize};
///Resource is an abstract entity that describes the common set of attributes shared by all concrete resources. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Resource entity and not the related ResourceRefOrValue class itself
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRefOrValue {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Configuration features
    #[serde(rename = "activationFeature")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation_feature: Option<Vec<Feature>>,
    ///ResourceAdministrativeStateType enumerations
    #[serde(rename = "administrativeState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<ResourceAdministrativeStateType>,
    ///Attachment reference. An attachment defines an attachment described by reference or by value linked to a specific entity. The polymorphic attributes @type, @schemaLocation & @referredType are related to the attachment entity and not the AttachmentRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
    ///Category of the concrete resource. e.g Gold, Silver for MSISDN concrete resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    ///free-text description of the resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///A date time( DateTime). The date till the resource is operating
    #[serde(rename = "endOperatingDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_operating_date: Option<crate::DateTime>,
    ///The URI for the object itself.
    pub href: String,
    ///Identifier of an instance of the resource. Required to be unique within the resource type.  Used in URIs as the identifier for specific instances of a type.
    pub id: String,
    ///A string used to give a name to the resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Note about the resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    ///ResourceOperationalStateType enumerations
    #[serde(rename = "operationalState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<ResourceOperationalStateType>,
    ///Related Entity reference. A related place defines a place described by reference or by value linked to a specific entity. The polymorphic attributes @type, @schemaLocation & @referredType are related to the place entity and not the RelatedPlaceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<RelatedPlaceRefOrValue>,
    /// Related Partys involved in the resource
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    /// A list of characteristics for the resource
    #[serde(rename = "resourceCharacteristic")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_characteristic: Option<Vec<Characteristic>>,
    /// A list of relationships between this resource and other resources
    #[serde(rename = "resourceRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_relationship: Option<Vec<ResourceRelationship>>,
    /**Resources are physical or non-physical components (or some combination of these) within an enterprise's infrastructure or inventory. They are typically consumed or used by services (for example a physical port assigned to a service) or contribute to the realization of a Product (for example, a SIM card). They can be drawn from the Application, Computing and Network domains, and include, for example, Network Elements, software, IT systems, content and information, and technology components.
    A ResourceSpecification is an abstract base class for representing a generic means for implementing a particular type of Resource. In essence, a ResourceSpecification defines the common attributes and relationships of a set of related Resources, while Resource defines a specific instance that is based on a particular ResourceSpecification.*/
    #[serde(rename = "resourceSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ResourceSpecificationRef>,
    ///ResourceStatusType enumerations
    #[serde(rename = "resourceStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<ResourceStatusType>,
    ///A field that identifies the specific version of an instance of a resource.
    #[serde(rename = "resourceVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_version: Option<String>,
    ///A date time( DateTime). The date from which the resource is operating
    #[serde(rename = "startOperatingDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_operating_date: Option<crate::DateTime>,
    ///ResourceUsageStateType enumerations
    #[serde(rename = "usageState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_state: Option<ResourceUsageStateType>,
}
impl std::fmt::Display for ResourceRefOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
