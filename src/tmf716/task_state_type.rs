use serde::{Serialize, Deserialize};
///Possible values for the state of a task
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskStateType {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "terminatedWithError")]
    TerminatedWithError,
    #[serde(rename = "inProgress")]
    InProgress,
    #[serde(rename = "done")]
    Done,
}
