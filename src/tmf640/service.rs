use serde::{Serialize, Deserialize};
use super::{
    Characteristic, ContextUpdate, ExternalIdentifier, Feature, IntentRefOrValue,
    RelatedEntityRefOrValue, RelatedPartyRefOrPartyRoleRef, RelatedPlaceRefOrValue,
    RelatedServiceOrderItem, ResourceRef, ServiceOperatingStatusType, ServiceRefOrValue,
    ServiceRelationship, ServiceSpecificationRef, ServiceStateType,
};
// use crate::common::extensible::Extensible;
use crate::common::note::Note;
use crate::common::entity::Entity;

///Service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Is it a customer facing or resource facing service
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    ///Free-text description of the service
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Date when the service ends
    #[serde(rename = "endDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<crate::DateTime>,
    ///A list of external identifiers assoicated with this service
    #[serde(rename = "externalIdentifier")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_identifier: Vec<ExternalIdentifier>,
    ///A list of feature associated with this service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feature: Vec<Feature>,
    ///If TRUE, this Service has already been started
    #[serde(rename = "hasStarted")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_started: Option<bool>,
    ///Intent Ref (if Intent already exists) or Value (if Intent be created or its details be presented)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<IntentRefOrValue>,
    ///If true, the service is a ServiceBundle which regroup a service hierachy. If false, the service is a 'atomic' service (hierachy leaf).
    #[serde(rename = "isBundle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bundle: Option<bool>,
    ///If FALSE and hasStarted is FALSE, this particular Service has NOT been enabled for use - if FALSE and hasStarted is TRUE then the service has failed
    #[serde(rename = "isServiceEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_service_enabled: Option<bool>,
    ///If TRUE, this Service can be changed without affecting any other services
    #[serde(rename = "isStateful")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_stateful: Option<bool>,
    ///Name of the service
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///A list of notes made on this service
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    ///Valid values for the Operating status of the service
    #[serde(rename = "operatingStatus")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_status: Option<ServiceOperatingStatusType>,
    ///Context update related to operating status changes
    #[serde(rename = "operatingStatusContextUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_status_context_update: Option<ContextUpdate>,
    ///A list of places (Place [*]). Used to define a place useful for the service (for example a geographical place whre the service is installed)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub place: Vec<RelatedPlaceRefOrValue>,
    ///A list of related entities in relationship with this service
    #[serde(rename = "relatedEntity")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_entity: Vec<RelatedEntityRefOrValue>,
    ///A list of related party references (RelatedParty [*]). A related party defines party or party role linked to a specific entity
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_party: Vec<RelatedPartyRefOrPartyRoleRef>,
    ///A list of characteristics that characterize this service (ServiceCharacteristic [*])
    #[serde(rename = "serviceCharacteristic")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_characteristic: Vec<Characteristic>,
    ///Date when the service was created (whatever its status).
    #[serde(rename = "serviceDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,
    ///A list of service order items related to this service
    #[serde(rename = "serviceOrderItem")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_order_item: Vec<RelatedServiceOrderItem>,
    ///A list of service relationships (ServiceRelationship [*]). Describes links with other service(s) in the inventory.
    #[serde(rename = "serviceRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_relationship: Vec<ServiceRelationship>,
    ///Reference to the specification that defines this service.
    #[serde(rename = "serviceSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecificationRef>,
    ///Business type of the service
    #[serde(rename = "serviceType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    ///Date when the service starts
    #[serde(rename = "startDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<crate::DateTime>,
    ///This attribute is an enumerated integer that indicates how the Service is started, such as: 0: Unknown; 1: Automatically by the managed environment; 2: Automatically by the owning device; 3: Manually by the Provider of the Service; 4: Manually by a Customer of the Provider; 5: Any of the above
    #[serde(rename = "startMode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_mode: Option<String>,
    ///Valid values for the lifecycle state of the service
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ServiceStateType>,
    ///A list of supporting resources (SupportingResource [*]).Note: only Service of type RFS can be associated with Resources
    #[serde(rename = "supportingResource")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_resource: Vec<ResourceRef>,
    ///A list of supporting services (SupportingService [*]). A collection of services that support this service (bundling, link CFS to RFS)
    #[serde(rename = "supportingService")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_service: Vec<ServiceRefOrValue>,
}
impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Service {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Service {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
