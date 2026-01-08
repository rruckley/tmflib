use serde::{Serialize, Deserialize};
use super::{
    AppointmentRef, OrderItemActionType, ServiceOrderItemErrorMessage,
    ServiceOrderItemRelationship, ServiceOrderItemStateType, ServiceRefOrValue,
};

///An item of a service order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOrderItem {
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
    ///action to be performed on the product
    pub action: OrderItemActionType,
    ///Refers an appointment, such as a Customer presentation or internal meeting or site visit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appointment: Option<AppointmentRef>,
    ///the error(s) cause an order item status change
    #[serde(rename = "errorMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Vec<ServiceOrderItemErrorMessage>>,
    ///Identifier of the individual line item
    pub id: String,
    ///Quantity ordered
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    ///A Service to be created defined by value or existing defined by reference. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the RelatedServiceRefOrValue class itself
    pub service: ServiceRefOrValue,
    ///A list of order items embedded to this order item
    #[serde(rename = "serviceOrderItem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order_item: Option<Vec<ServiceOrderItem>>,
    ///A list of order items related to this order item
    #[serde(rename = "serviceOrderItemRelationship")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order_item_relationship: Option<Vec<ServiceOrderItemRelationship>>,
    ///Possible values for the state of the order item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ServiceOrderItemStateType>,
}
impl std::fmt::Display for ServiceOrderItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
