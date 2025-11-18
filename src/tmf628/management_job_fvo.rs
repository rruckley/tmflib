use serde::{Serialize, Deserialize};
use super::{
    AdministrativeState, DataAccessEndpointFvo, Entity, ExecutionStateType,
    FileTransferDataFvo, ScheduleDefinitionFvo,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagementJobFvo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///This is enumeration for Administrative state
    #[serde(rename = "adminState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_state: Option<AdministrativeState>,
    ///The measurement job creation time.
    #[serde(rename = "creationTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "dataAccessEndpoint")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data_access_endpoint: Vec<DataAccessEndpointFvo>,
    ///Possible values for the state of the execution
    #[serde(rename = "executionState")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_state: Option<ExecutionStateType>,
    #[serde(rename = "fileTransferData")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_transfer_data: Vec<FileTransferDataFvo>,
    ///The ID of the management job.
    #[serde(rename = "jobId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    ///The priority of the management job. The way the management application will use the JobPriority to schedule job execution is application specific and outside the scope. Integer, limited to a range of 1 to 10.
    #[serde(rename = "jobPriority")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_priority: Option<i64>,
    ///The last time that a measurement job was modified.
    #[serde(rename = "lastModifiedTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "scheduleDefinition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedule_definition: Vec<ScheduleDefinitionFvo>,
}
impl std::fmt::Display for ManagementJobFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagementJobFvo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for ManagementJobFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
