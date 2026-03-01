use serde::{Serialize, Deserialize};
use super::ResourceReservationRef;
///Skipped properties: id,href,state,effectiveCancellationDate
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CancelResourceReservationCreate {
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
    ///Reason why the reservation is cancelled.
    #[serde(rename = "cancellationReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    ///Date when the submitter wants the reservation to be cancelled
    #[serde(rename = "requestedCancellationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_cancellation_date: Option<String>,
    #[serde(rename = "resourceReservation")]
    pub resource_reservation: ResourceReservationRef,
}
impl std::fmt::Display for CancelResourceReservationCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
