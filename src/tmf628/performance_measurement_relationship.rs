use serde::{Serialize, Deserialize};
use super::{Entity, PerformanceMeasurementRefOrValue, TimePeriod};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMeasurementRelationship {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///A PerformanceMeasurement to be created defined by value or existing defined by reference. The polymorphic attributes @type, @schemaLocation & @referredType are related to the product entity and not the RelatedProductRefOrValue class itself
    #[serde(rename = "relatedMeasurement")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_measurement: Option<PerformanceMeasurementRefOrValue>,
    ///
    #[serde(rename = "relationshipType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    ///The association role for this PerformanceMeasurement
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for PerformanceMeasurementRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PerformanceMeasurementRelationship {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PerformanceMeasurementRelationship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
