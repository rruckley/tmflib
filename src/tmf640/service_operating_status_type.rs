use serde::{Serialize, Deserialize};
///Valid values for the Operating status of the service
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceOperatingStatusType {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "configured")]
    Configured,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "limited")]
    Limited,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "unknown")]
    Unknown,
}
