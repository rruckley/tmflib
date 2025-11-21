use serde::{Serialize, Deserialize};
use super::{
    CollectionType, IndicatorType, PerformanceIndicatorSpecRelationship,
};
use crate::{
    common::entity::Entity,
    TimePeriod,
};

/// Performance Indicator Specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceIndicatorSpecification {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///This is enumeration for CollectionType state
    #[serde(rename = "collectionType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection_type: Option<CollectionType>,
    ///A step-by-step procedure used to calculate the value of PerformanceIndicator.
    #[serde(rename = "derivationAlgorithm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derivation_algorithm: Option<String>,
    ///For simple calculations, the method used to calculate the value of a PerformanceIndicator, such as average, minimum, maximum, sum and so forth.
    #[serde(rename = "derivationMethod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derivation_method: Option<String>,
    ///A narrative that explains in detail what the PerformanceIndicatorSpecification is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///A grouping or set of PerformanceIndicatorSpecifications that are classified together because of common characteristics, such as technology specific, service specific, or technology/service independent.
    #[serde(rename = "indicatorCategory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_category: Option<String>,
    ///This is enumeration for Indicator Type
    #[serde(rename = "indicatorType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_type: Option<IndicatorType>,
    ///The unit by which the indicator is measured. For example, seconds, KBs, rate per second, etc.
    #[serde(rename = "indicatorUnit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indicator_unit: Option<String>,
    ///A word, term, or phrase by which a PerformanceIndicatorSpecification is known and distinguished from other PerformanceIndicatorSpecifications.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Relationships to other PerformanceIndicatorSpecification objects.
    #[serde(rename = "performanceIndicatorSpecRelationship")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performance_indicator_spec_relationship: Vec<
        PerformanceIndicatorSpecRelationship,
    >,
    ///The point of view for the PerformanceIndicatorSpecification, such as a single user instance or aggregation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub perspective: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for PerformanceIndicatorSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for PerformanceIndicatorSpecification {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for PerformanceIndicatorSpecification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
