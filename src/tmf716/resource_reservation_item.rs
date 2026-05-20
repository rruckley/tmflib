use super::{
    Capacity, RelatedEntity, RelatedPartyRef, ReservationItemActionType, ReservationItemStateType,
    ResourceRefOrValue, TimePeriod,
};
use serde::{Deserialize, Serialize};

///`ResourceReservationItem` struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceReservationItem {
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
    ///action to be performed on the entity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ReservationItemActionType>,
    ///Specific ability of an entity measured in quantity and units of quantity over an extended period.
    #[serde(rename = "appliedCapacity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_capacity: Option<Capacity>,
    ///Specific ability of an entity measured in quantity and units of quantity over an extended period.
    #[serde(rename = "capacityDemand")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity_demand: Option<Capacity>,
    ///A string. Identifier of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Quantity of the reservation item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    ///A string used to give a name to the reservation item
    #[serde(rename = "relatedEntity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_entity: Option<Vec<RelatedEntity>>,
    ///A string used to give a name to the reservation item
    #[serde(rename = "relatedParty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedPartyRef>>,
    ///A string used to give a name to the reservation item
    #[serde(rename = "reservationItem")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reservation_item: Option<Vec<Self>>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "reservationPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reservation_period: Option<TimePeriod>,
    ///Resource is an abstract entity that describes the common set of attributes shared by all concrete resources. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Resource entity and not the related `ResourceRefOrValue` class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ResourceRefOrValue>,
    /// The state of the reservation item, such as 'reserved', 'completed', 'cancelled'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ReservationItemStateType>,
    ///The date and time the state changed.
    #[serde(rename = "stateChangeDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_change_date: Option<crate::DateTime>,
    ///The reason for changing the state
    #[serde(rename = "stateChangeReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_change_reason: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for ResourceReservationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
