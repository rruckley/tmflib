use serde::{Serialize, Deserialize};
use super::ServiceOrderItemRef;
///A ServiceOrderErrorMessage represents an error that causes a status change in a service order.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceOrderErrorMessage {
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
    ///error code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    ///More details and corrective actions related to the error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    ///Explanation of the reason for the error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    ///URI of documentation describing the error
    #[serde(rename = "referenceError")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_error: Option<String>,
    ///A list of order item references corresponded to this error
    #[serde(rename = "serviceOrderItem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order_item: Option<Vec<ServiceOrderItemRef>>,
    ///error code extension like sys-ABC-2001
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///Date when the error happened
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::DateTime>,
}
impl std::fmt::Display for ServiceOrderErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
