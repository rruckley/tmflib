use serde::{Serialize, Deserialize};
///Reference schema .
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReferenceFvoFvoFvoFvoFvo {
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///Identifier of the referred entity.
    pub id: String,
    ///Name of the referred entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl std::fmt::Display for ReferenceFvoFvoFvoFvoFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
