use serde::{Serialize, Deserialize};
///Policy is a set of rules that are used to manage and control the state and state transitions of one or more managed objects.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Policy {}
impl std::fmt::Display for Policy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
