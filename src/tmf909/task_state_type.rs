use serde::{Serialize, Deserialize};
///Possible values for the state of a task
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStateType {
    ///The task state is accepted
    #[serde(rename = "accepted")]
    Accepted,
    ///The task state is rejected
    #[serde(rename = "terminatedWithError")]
    TerminatedWithError,
    ///The task state is in progress
    #[serde(rename = "inProgress")]
    InProgress,
    ///The task state is done
    #[serde(rename = "done")]
    Done,
}
