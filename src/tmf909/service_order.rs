use serde::{Serialize, Deserialize};
use super::{
    ExternalReference, Note, RelatedParty, ServiceOrderErrorMessage, ServiceOrderItem,
    ServiceOrderJeopardyAlert, ServiceOrderMilestone, ServiceOrderRelationship,
    ServiceOrderStateType,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOrder {
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
    ///Effective delivery date amended by the provider
    #[serde(rename = "completionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<crate::DateTime>,
    ///A free-text description of the service order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///the error(s) cause an order status change
    #[serde(rename = "errorMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Vec<ServiceOrderErrorMessage>>,
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
    ///Hyperlink to access the order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///ID created on repository side
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A list of jeopardy alerts related to this order
    #[serde(rename = "jeopardyAlert")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jeopardy_alert: Option<Vec<ServiceOrderJeopardyAlert>>,
    ///A list of milestones related to this order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Vec<ServiceOrderMilestone>>,
    ///Extra-information about the order; e.g. useful to add extra delivery information that could be useful for a human process
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    ///Contact attached to the order to send back information regarding this order
    #[serde(rename = "notificationContact")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_contact: Option<String>,
    #[serde(rename = "orderDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_date: Option<crate::DateTime>,
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
    ///Date when the order was started for processing
    #[serde(rename = "startDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<crate::DateTime>,
    ///Possible values for the state of the order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ServiceOrderStateType>,
}
impl std::fmt::Display for ServiceOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
