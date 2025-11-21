use serde::{Serialize, Deserialize};
///Place reference.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaceRef {}
impl std::fmt::Display for PlaceRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
