use serde::{Deserialize, Serialize};
///Possible values for the state of the change request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChangeRequestStatusType {
    ///The change request is in the initial state and has been acknowledged
    #[serde(rename = "acknowledged")]
    Acknowledged,
    ///The change request is in the initial state and is waiting for authorization
    #[serde(rename = "requestForAuthorization")]
    RequestForAuthorization,
    ///The change request is in the initial state and  is waiting for approval
    #[serde(rename = "waitForApproval")]
    WaitForApproval,
    ///The change request is in the initial state and has been approved
    #[serde(rename = "approved")]
    Approved,
    ///The change request is in the initial state and has been scheduled
    #[serde(rename = "scheduled")]
    Scheduled,
    ///The change request has been implemented and is waiting for post implementation review
    #[serde(rename = "postImplementationReview")]
    PostImplementationReview,
    ///The change request is excuting fallback execution plan due to failed implementation
    #[serde(rename = "fallbackExecution")]
    FallbackExecution,
    /// The Change has been cancelled and will not be implemented. The change request is in the final state.
    #[serde(rename = "cancelled")]
    Cancelled,
    /// The change has been rejected and will not be implemented. The change request is in the final state.
    #[serde(rename = "rejected")]
    Rejected,
    /// The change is n progress and is being implemented. The change request is in the final state.
    #[serde(rename = "inProgress")]
    InProgress,
    /// The change has failed and will not be implemented. The change request is in the final state.
    #[serde(rename = "failed")]
    Failed,
    /// The change has been completed and is in the final state.
    #[serde(rename = "completed")]
    Completed,
}
