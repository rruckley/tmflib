use serde::{Deserialize, Serialize};
///Valid values for the Operating status of the service
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceOperatingStatusType {
    ///The service is being installed or activated
    #[serde(rename = "pending")]
    Pending,
    ///The service is being configured
    #[serde(rename = "configured")]
    Configured,
    ///The service is being started
    #[serde(rename = "starting")]
    Starting,
    ///The service is running
    #[serde(rename = "running")]
    Running,
    ///The service is being paused
    #[serde(rename = "degraded")]
    Degraded,
    ///The service is being paused
    #[serde(rename = "failed")]
    Failed,
    ///The service is being limited
    #[serde(rename = "limited")]
    Limited,
    ///The service is being stopped
    #[serde(rename = "stopping")]
    Stopping,
    ///The service is stopped
    #[serde(rename = "stopped")]
    Stopped,
    ///The service is being suspended
    #[serde(rename = "unknown")]
    Unknown,
}
