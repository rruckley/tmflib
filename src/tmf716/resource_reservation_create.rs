use serde::{Serialize, Deserialize};
use super::{
    ChannelRef, RelatedEntity, RelatedPartyRef, ReservationStateType,
    ResourceReservationItem, TimePeriod,
};
///Skipped properties: id,href
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceReservationCreate {
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
    ///Date when the reservation is cancelled.
    #[serde(rename = "cancellationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_date: Option<String>,
    ///Reason why the reservation is cancelled.
    #[serde(rename = "cancellationReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    ///The channel to which the resource reference to. e.g. channel for selling product offerings, channel for opening a trouble ticket etc..
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<ChannelRef>,
    ///Date when the reservation was completed
    #[serde(rename = "completionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    ///Date when the reservation was created
    #[serde(rename = "creationDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<chrono::DateTime<chrono::Utc>>,
    ///A string. free-text description of the reservation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Expected completion date amended by the provider
    #[serde(rename = "expectedCompletionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_completion_date: Option<String>,
    #[serde(rename = "relatedEntity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_entity: Option<Vec<RelatedEntity>>,
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedPartyRef>>,
    ///Requested completion date from the requestor perspective
    #[serde(rename = "requestedCompletionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_completion_date: Option<String>,
    ///Reservation fulfillment start date wished by the requestor. This is used when, for any reason, requestor cannot allow the reservation to begin before a specific date.
    #[serde(rename = "requestedStartDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_start_date: Option<String>,
    #[serde(rename = "reservationItem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reservation_item: Option<Vec<ResourceReservationItem>>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "reservationPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reservation_period: Option<TimePeriod>,
    ///
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ReservationStateType>,
    ///The date and time the state changed.
    #[serde(rename = "stateChangeDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_change_date: Option<chrono::DateTime<chrono::Utc>>,
    ///The reason for changing the state
    #[serde(rename = "stateChangeReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for ResourceReservationCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
