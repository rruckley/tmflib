use serde::{Deserialize, Serialize};
///      Reference to an entity FVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityRefOrValueFvo {}
impl std::fmt::Display for EntityRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
