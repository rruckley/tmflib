use serde::{Deserialize, Serialize};
///give an attribute used for filtering
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataFilterAttribute {
    ///Data type of the value of the attribute
    #[serde(rename = "valueType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}
impl std::fmt::Display for DataFilterAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
