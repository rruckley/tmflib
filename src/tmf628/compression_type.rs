use super::CompressionEnumType;
use serde::{Deserialize, Serialize};
///File compression type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionType {
    ///Enumeration of supported compressions. All extensions allowed.
    #[serde(rename = "compressionEnumType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_enum_type: Option<CompressionEnumType>,
}
impl std::fmt::Display for CompressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
