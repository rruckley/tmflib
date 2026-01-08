use serde::{Serialize, Deserialize};
use super::{
    ExternalReference, RelatedParty, ServiceOrderItem, ServiceOrderRelationship,
};
use crate::common::note::Note;
///Skipped properties: id,href,orderDate,completionDate,expectedCompletionDate,startDate,state,jeopardyAlert,errorMessage,milestone
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceOrderCreate {
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
    ///Date when the order is cancelled. This is used when order is cancelled.
    #[serde(rename = "cancellationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_date: Option<crate::DateTime>,
    ///Reason why the order is cancelled. This is used when order is cancelled.
    #[serde(rename = "cancellationReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    ///Used to categorize the order, useful for the OM system, such as: Broadband, TVOption
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    ///A free-text description of the service order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///ID given by the consumer to facilitate searches
    #[serde(rename = "externalId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    ///List of external references associated with this order
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_order_item: Vec<ServiceOrderItem>,
}
impl std::fmt::Display for ServiceOrderCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
