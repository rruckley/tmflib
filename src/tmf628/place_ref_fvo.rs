use serde::{Serialize, Deserialize};
///Place reference.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaceRefFvo {}
impl std::fmt::Display for PlaceRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
