use super::{ApplicableTimePeriod, CapacityAmount, CapacityRef, CapacitySpecRef, PlaceRefOrValue};
use serde::{Deserialize, Serialize};
///Specific ability of an entity measured in quantity and units of quantity over an extended period.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Capacity {
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
    ///The period of time for which Capacity or CapacityDemand applies.
    #[serde(rename = "applicableTimePeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicable_time_period: Option<Vec<ApplicableTimePeriod>>,
    ///Quantity that defines the Capacity.
    #[serde(rename = "capacityAmount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity_amount: Option<CapacityAmount>,
    ///A reference to a Capacity.
    #[serde(rename = "capacitySpec")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity_spec: Option<CapacitySpecRef>,
    ///A place defines a place described by reference or by value linked to a specific entity. The polymorphic attributes @type, @schemaLocation & @referredType are related to the place entity and not the RelatedPlaceRefOrValue class itself
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place: Option<PlaceRefOrValue>,
    ///An indicator that specifies whether the capacity is planned or actual.
    #[serde(rename = "plannedOrActualCapacity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planned_or_actual_capacity: Option<String>,
    ///related capacity.
    #[serde(rename = "relatedCapacity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_capacity: Option<Vec<CapacityRef>>,
}
impl std::fmt::Display for Capacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
