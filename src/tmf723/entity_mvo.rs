use serde::{Serialize, Deserialize};
///Base entity schema for use in TMForum Open-APIs. Property.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityMvo {}
impl std::fmt::Display for EntityMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
