use serde::{Serialize, Deserialize};
use super::{
    AttachmentRefFvo, CharacteristicFvo, ExternalIdentifierFvo, FeatureFvo,
    IntentRefFvo, NoteFvo, RelatedPartyRefOrPartyRoleRefFvo, RelatedPlaceRefFvo,
    RelatedResourceOrderItemFvo, ResourceAdministrativeStateType,
    ResourceOperationalStateType, ResourceRefOrValueFvo, ResourceRelationshipFvo,
    ResourceSpecificationRefFvo, ResourceStatusType, ResourceUsageStateType,
};
use crate::{
    common::entity::Entity,
    TimePeriod,
};

///Resource FVO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceFvo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Configuration features
    #[serde(rename = "activationFeature")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activation_feature: Vec<FeatureFvo>,
    ///ResourceAdministrativeStateType enumerations; values defined by ITU X.731: 'locked': The resource is administratively prohibited from performing services for its users; 'shutdown': Use of the resource is administratively permitted to existing instances of use only. While the system remains in the shutting down state the manager may at any time cause the managed object to revert to the unlocked state; 'unlocked': The resource is administratively permitted to perform services for its users. This is independent of its inherent operability.
    #[serde(rename = "administrativeState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<ResourceAdministrativeStateType>,
    ///A list of attachments associated with the resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachment: Vec<AttachmentRefFvo>,
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
    ///An identification of this resource that is owned by or originates in a software system different from the current system. The structure identifies the system itself, the nature of the resource within the system and the unique ID of the resource within the system. It is anticipated that multiple external IDs can be held for a single resource, e.g. if the resource passed through multiple systems on the way to the current system.
    #[serde(rename = "externalIdentifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_identifier: Vec<ExternalIdentifierFvo>,
    ///Intent reference, for when Intent is used by other entities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<IntentRefFvo>,
    ///the name of the resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A list of notes associated with the resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<NoteFvo>,
    ///ResourceOperationalStateType enumerations; values defined by ITU X.731: 'disable': The resource is totally inoperable and unable to provide service to the user(s); 'enable': The resource is partially or fully operable and available for use.
    #[serde(rename = "operationalState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<ResourceOperationalStateType>,
    ///A list of places associated with the resource
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub place: Vec<RelatedPlaceRefFvo>,
    ///A list of parties or party roles associated with the resource
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyRefOrPartyRoleRefFvo>,
    ///A list of characteristics of the resource
    #[serde(rename = "resourceCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_characteristic: Vec<CharacteristicFvo>,
    ///A list of resource order items related to this resource
    #[serde(rename = "resourceOrderItem")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    ///ResourceOrderItem [*]. A collection of resource order items that are related to this resource.
    pub resource_order_item: Vec<RelatedResourceOrderItemFvo>,
    ///A list of relationships to other resources
    #[serde(rename = "resourceRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_relationship: Vec<ResourceRelationshipFvo>,
    ///Reference to the specification that is used to instantiate the resource
    #[serde(rename = "resourceSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ResourceSpecificationRefFvo>,
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
    ///A list of supporting resources (SupportingResource [*]). A collection of resources that support this resource (bundling, link ResourceSpecification)
    #[serde(rename = "supportingResource")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_resource: Vec<ResourceRefOrValueFvo>,
    ///ResourceUsageStateType enumerations; values defined by ITU X.731: 'idle': The resource is not currently in use; 'active': The resource is in use, and has sufficient spare operating capacity to provide for additional users simultaneously; 'busy': The resource is in use, but it has no spare operating capacity to provide for additional users at this instant.
    #[serde(rename = "usageState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_state: Option<ResourceUsageStateType>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for ResourceFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ResourceFvo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for ResourceFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
