use serde::{Serialize, Deserialize};
///Container for PolicyEvent Reference or PolicyEvent object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyEventRefOrValue {}
impl std::fmt::Display for PolicyEventRefOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
