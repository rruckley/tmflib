use serde::{Deserialize, Serialize};
///Place reference.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaceRefMvo {}
impl std::fmt::Display for PlaceRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
