use serde::{Serialize, Deserialize};
use super::{
    Granularity, ManagementJob, MonitoredClassCriteria, MonitoredInstancesCriteria,
    PerformanceIndicatorGroupSpecification, PerformanceIndicatorSpecificationRefOrValue,
    TrackingRecord,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementJob {
    #[serde(flatten)]
    pub management_job: ManagementJob,
    ///The identifier of the application that consumes performance indicators.
    #[serde(rename = "consumingApplicationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consuming_application_id: Option<String>,
    ///Sampling rate of the collection or production of performance indicators.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<Granularity>,
    #[serde(rename = "monitoredClassCriteria")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitored_class_criteria: Option<MonitoredClassCriteria>,
    #[serde(rename = "monitoredInstancesCriteria")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitored_instances_criteria: Option<MonitoredInstancesCriteria>,
    #[serde(rename = "performanceIndicatorGroupSpecification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performance_indicator_group_specification: Vec<
        PerformanceIndicatorGroupSpecification,
    >,
    #[serde(rename = "performanceIndicatorSpecification")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub performance_indicator_specification: Vec<
        PerformanceIndicatorSpecificationRefOrValue,
    >,
    ///The identifier of the application that produces performance indicators.
    #[serde(rename = "producingApplicationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producing_application_id: Option<String>,
    #[serde(rename = "trackingRecord")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracking_record: Vec<TrackingRecord>,
}
impl std::fmt::Display for MeasurementJob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for MeasurementJob {
    type Target = ManagementJob;
    fn deref(&self) -> &Self::Target {
        &self.management_job
    }
}
impl std::ops::DerefMut for MeasurementJob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.management_job
    }
}
