use serde::{Serialize, Deserialize};
use super::{
    MeasurementCollectionJobRef, PerformanceMeasurementRelationship,
};
use crate::{
    common::entity::Entity,
    TimePeriod,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMeasurement {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///A free-text description of the performance measurement
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Reference to a MeasurementCollectionJob
    #[serde(rename = "measurementCollectionJob")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurement_collection_job: Option<MeasurementCollectionJobRef>,
    ///related Performance measurements array
    #[serde(rename = "relatedMeasurement")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_measurement: Vec<PerformanceMeasurementRelationship>,
    ///The optional tag object attached to this entire measurement
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<serde_json::Value>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for PerformanceMeasurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PerformanceMeasurement {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PerformanceMeasurement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
