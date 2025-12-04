use super::PackingEnumType;
use serde::{Deserialize, Serialize};
///Specify if the output file(s) are to be packed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackingType {
    ///Enumeration of supported packing/packaging. All extensions allowed.
    #[serde(rename = "packingEnumType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packing_enum_type: Option<PackingEnumType>,
}
impl std::fmt::Display for PackingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
