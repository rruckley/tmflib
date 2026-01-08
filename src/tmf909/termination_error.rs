use serde::{Serialize, Deserialize};
///This indicates an error that caused a qualificationItem to be terminated.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminationError {
    ///Unique identifier of the termination error
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Text to describe the termination error - for example: Unable to proceed to qualification because incomplete information provided
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl std::fmt::Display for TerminationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
