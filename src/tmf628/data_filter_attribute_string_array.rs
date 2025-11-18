use serde::{Serialize, Deserialize};
use super::DataFilterAttribute;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataFilterAttributeStringArray {
    ///give an attribute used for filtering
    #[serde(flatten)]
    pub data_filter_attribute: DataFilterAttribute,
    ///an array of field names
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl std::fmt::Display for DataFilterAttributeStringArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DataFilterAttributeStringArray {
    type Target = DataFilterAttribute;
    fn deref(&self) -> &Self::Target {
        &self.data_filter_attribute
    }
}
impl std::ops::DerefMut for DataFilterAttributeStringArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data_filter_attribute
    }
}
