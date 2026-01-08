use serde::{Serialize, Deserialize};
use super::ServiceOrderRef;
/**Request for cancellation an existing Service order
Skipped properties: id,href,state,effectiveCancellationDate,completionMessage,errorMessage*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancelServiceOrderCreate {
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
    ///Reason why the order is cancelled.
    #[serde(rename = "cancellationReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    ///Date when the submitter wants the order to be cancelled
    #[serde(rename = "requestedCancellationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_cancellation_date: Option<crate::DateTime>,
    ///Service Order reference. Useful to understand the which was the Service order through which the service was instantiated in the service inventory
    #[serde(rename = "serviceOrder")]
    pub service_order: ServiceOrderRef,
}
impl std::fmt::Display for CancelServiceOrderCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
