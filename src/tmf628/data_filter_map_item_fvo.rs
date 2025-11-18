use serde::{Serialize, Deserialize};
use super::{DataFilterAttributeStringArray, DataFilterTemplateFvo};
///definition of a field used for filtering template with the associated attributes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataFilterMapItemFvo {
    #[serde(rename = "filterTemplate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_template: Option<DataFilterTemplateFvo>,
    #[serde(rename = "stringArray")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_array: Option<DataFilterAttributeStringArray>,
}
impl std::fmt::Display for DataFilterMapItemFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
