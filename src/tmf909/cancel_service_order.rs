use serde::{Serialize, Deserialize};
use super::{ErrorMessage, ServiceOrderRef, TaskStateType};
///Request for cancellation an existing Service order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelServiceOrder {
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
    ///an optional message describing the completion of the task if it is done as expected or it is denied for a reason (like order in an state of PoNR).
    #[serde(rename = "completionMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_message: Option<String>,
    ///Date when the order is cancelled.
    #[serde(rename = "effectiveCancellationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_cancellation_date: Option<crate::DateTime>,
    ///represents an Error
    #[serde(rename = "errorMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<ErrorMessage>,
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///unique identifier
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Date when the submitter wants the order to be cancelled
    #[serde(rename = "requestedCancellationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_cancellation_date: Option<crate::DateTime>,
    ///Service Order reference. Useful to understand the which was the Service order through which the service was instantiated in the service inventory
    #[serde(rename = "serviceOrder")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_order: Option<ServiceOrderRef>,
    ///Possible values for the state of a task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<TaskStateType>,
}
impl std::fmt::Display for CancelServiceOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
