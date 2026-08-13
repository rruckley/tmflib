use serde::{Serialize, Deserialize};
///Possible values for the state of the change request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChangeRequestStatusType {
    #[serde(rename = "acknowledged")]
    Acknowledged,
    #[serde(rename = "requestForAuthorization")]
    RequestForAuthorization,
    #[serde(rename = "waitForApproval")]
    WaitForApproval,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "postImplementationReview")]
    PostImplementationReview,
    #[serde(rename = "fallbackExecution")]
    FallbackExecution,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "completed")]
    Completed,
}
