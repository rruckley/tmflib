use serde::{Serialize, Deserialize};
use super::{DataAccessEndpointMvo, Entity, FileTransferDataMvo, ScheduleDefinitionMvo};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagementJobMvo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    #[serde(rename = "dataAccessEndpoint")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data_access_endpoint: Vec<DataAccessEndpointMvo>,
    #[serde(rename = "fileTransferData")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_transfer_data: Vec<FileTransferDataMvo>,
    #[serde(rename = "scheduleDefinition")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schedule_definition: Vec<ScheduleDefinitionMvo>,
}
impl std::fmt::Display for ManagementJobMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ManagementJobMvo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for ManagementJobMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
