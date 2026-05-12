use serde::{Serialize, Deserialize};
///The reference object to the schema and type of target entity which is described by a specification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TargetEntitySchemaMvo {
    ///This field provides a link to the schema describing the target entity
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///Class type of the target entity
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for TargetEntitySchemaMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
