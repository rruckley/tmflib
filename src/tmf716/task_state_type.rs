use serde::{Deserialize, Serialize};
///Possible values for the state of a task
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStateType {
    ///The task is in an accepted state and is waiting to be processed.
    #[serde(rename = "accepted")]
    Accepted,
    ///The task is in a rejected state and will not be processed.
    #[serde(rename = "terminatedWithError")]
    TerminatedWithError,
    ///The task is in a rejected state and will not be processed.
    #[serde(rename = "inProgress")]
    InProgress,
    ///The task is in a rejected state and will not be processed.
    #[serde(rename = "done")]
    Done,
}
