use serde::{Serialize, Deserialize};
///A PolicyEvent is an occurrence of an important event or multiple events, and can be used to trigger the evaluation of a Policy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyEventMvo {}
impl std::fmt::Display for PolicyEventMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
