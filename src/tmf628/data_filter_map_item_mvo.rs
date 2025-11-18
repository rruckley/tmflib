use serde::{Serialize, Deserialize};
use super::{DataFilterAttributeStringArray, DataFilterTemplateMvo};
///definition of a field used for filtering template with the associated attributes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataFilterMapItemMvo {
    #[serde(rename = "filterTemplate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_template: Option<DataFilterTemplateMvo>,
    #[serde(rename = "stringArray")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_array: Option<DataFilterAttributeStringArray>,
}
impl std::fmt::Display for DataFilterMapItemMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
