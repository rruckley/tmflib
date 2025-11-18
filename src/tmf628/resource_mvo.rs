use serde::{Serialize, Deserialize};
use super::{
    AttachmentRefMvo, CharacteristicMvo, Entity, ExternalIdentifierMvo, FeatureMvo,
    IntentRefMvo, NoteMvo, RelatedPartyRefOrPartyRoleRefMvo, RelatedPlaceRefMvo,
    RelatedResourceOrderItemMvo, ResourceAdministrativeStateType,
    ResourceOperationalStateType, ResourceRefOrValueMvo, ResourceRelationshipMvo,
    ResourceSpecificationRefMvo, ResourceStatusType, ResourceUsageStateType, TimePeriod,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMvo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Configuration features
    #[serde(rename = "activationFeature")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activation_feature: Vec<FeatureMvo>,
    ///ResourceAdministrativeStateType enumerations; values defined by ITU X.731: 'locked': The resource is administratively prohibited from performing services for its users; 'shutdown': Use of the resource is administratively permitted to existing instances of use only. While the system remains in the shutting down state the manager may at any time cause the managed object to revert to the unlocked state; 'unlocked': The resource is administratively permitted to perform services for its users. This is independent of its inherent operability.
    #[serde(rename = "administrativeState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_state: Option<ResourceAdministrativeStateType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachment: Vec<AttachmentRefMvo>,
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
    pub external_identifier: Vec<ExternalIdentifierMvo>,
    ///Intent reference, for when Intent is used by other entities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<IntentRefMvo>,
    ///the name of the resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<NoteMvo>,
    ///ResourceOperationalStateType enumerations; values defined by ITU X.731: 'disable': The resource is totally inoperable and unable to provide service to the user(s); 'enable': The resource is partially or fully operable and available for use.
    #[serde(rename = "operationalState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<ResourceOperationalStateType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub place: Vec<RelatedPlaceRefMvo>,
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyRefOrPartyRoleRefMvo>,
    #[serde(rename = "resourceCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_characteristic: Vec<CharacteristicMvo>,
    ///A list of resource order items related to this resource
    #[serde(rename = "resourceOrderItem")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_order_item: Vec<RelatedResourceOrderItemMvo>,
    #[serde(rename = "resourceRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_relationship: Vec<ResourceRelationshipMvo>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ResourceSpecificationRefMvo>,
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
    pub supporting_resource: Vec<ResourceRefOrValueMvo>,
    ///ResourceUsageStateType enumerations; values defined by ITU X.731: 'idle': The resource is not currently in use; 'active': The resource is in use, and has sufficient spare operating capacity to provide for additional users simultaneously; 'busy': The resource is in use, but it has no spare operating capacity to provide for additional users at this instant.
    #[serde(rename = "usageState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_state: Option<ResourceUsageStateType>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for ResourceMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ResourceMvo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for ResourceMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
