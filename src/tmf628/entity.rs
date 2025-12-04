use serde::{Deserialize, Serialize};
///Base entity schema for use in TMForum Open-APIs. Property.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Entity {}
impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
