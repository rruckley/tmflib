use serde::{Serialize, Deserialize};
use super::{
    ExternalReference, Note, RelatedParty, ServiceOrderItem, ServiceOrderRelationship,
    ServiceOrderStateType,
};
///Skipped properties: id,href,orderDate,jeopardyAlert,errorMessage,milestone,@baseType,@schemaLocation,@type,cancellationDate,cancellationReason,category,completionDate,startDate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOrderUpdate {
    ///A free-text description of the service order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Expected delivery date amended by the provider
    #[serde(rename = "expectedCompletionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_completion_date: Option<crate::DateTime>,
    ///ID given by the consumer to facilitate searches
    #[serde(rename = "externalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "externalReference")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<Vec<ExternalReference>>,
    ///Extra-information about the order; e.g. useful to add extra delivery information that could be useful for a human process
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    ///Contact attached to the order to send back information regarding this order
    #[serde(rename = "notificationContact")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_contact: Option<String>,
    ///A list of service orders related to this order (e.g. prerequisite, dependent on)
    #[serde(rename = "orderRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_relationship: Option<Vec<ServiceOrderRelationship>>,
    ///Can be used by consumers to prioritize orders in a Service Order Management system
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    ///A list of parties which are involved in this order and the role they are playing
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,
    ///Requested delivery date from the requestors perspective
    #[serde(rename = "requestedCompletionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_completion_date: Option<crate::DateTime>,
    ///Order start date wished by the requestor
    #[serde(rename = "requestedStartDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_start_date: Option<crate::DateTime>,
    ///A list of service order items to be processed by this order
    #[serde(rename = "serviceOrderItem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order_item: Option<Vec<ServiceOrderItem>>,
    ///Possible values for the state of the order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ServiceOrderStateType>,
}
impl std::fmt::Display for ServiceOrderUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
