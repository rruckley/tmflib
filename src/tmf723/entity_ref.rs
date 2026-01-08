use serde::{Serialize, Deserialize};
///Entity reference schema to be use for all entityRef class.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityRef {
    ///Hyperlink reference
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}
impl std::fmt::Display for EntityRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
