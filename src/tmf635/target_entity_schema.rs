use serde::{Deserialize, Serialize};
///The reference object to the schema and type of target entity which is described by a specification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TargetEntitySchema {
    ///This field provides a link to the schema describing the target entity
    #[serde(rename = "@schemaLocation")]
    pub schema_location: String,
    ///Class type of the target entity
    #[serde(rename = "@type")]
    pub type_: String,
}
impl std::fmt::Display for TargetEntitySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
