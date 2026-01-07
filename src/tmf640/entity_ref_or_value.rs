use serde::{Deserialize, Serialize};
/// This is an entity which may be represented either as a reference or as an inline value.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityRefOrValue {}
impl std::fmt::Display for EntityRefOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
