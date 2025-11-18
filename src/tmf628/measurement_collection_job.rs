use serde::{Serialize, Deserialize};
use super::{DataFilterMap, MeasurementJob, ReportingPeriod};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementCollectionJob {
    #[serde(flatten)]
    pub measurement_job: MeasurementJob,
    #[serde(rename = "jobCollectionFilter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_collection_filter: Option<DataFilterMap>,
    ///True if the job is a single job to be executed immediately in which case the reportingPeriod and scheduleDefinition would not be applicable, false otherwise
    #[serde(rename = "jobOnDemand")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_on_demand: Option<bool>,
    #[serde(rename = "outputFormat")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    ///Possible values for the reporting period
    #[serde(rename = "reportingPeriod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reporting_period: Option<ReportingPeriod>,
    #[serde(rename = "searchTaskFilter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_task_filter: Option<DataFilterMap>,
}
impl std::fmt::Display for MeasurementCollectionJob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for MeasurementCollectionJob {
    type Target = MeasurementJob;
    fn deref(&self) -> &Self::Target {
        &self.measurement_job
    }
}
impl std::ops::DerefMut for MeasurementCollectionJob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.measurement_job
    }
}
