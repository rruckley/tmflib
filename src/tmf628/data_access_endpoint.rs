use serde::{Serialize, Deserialize};
use super::{DataFilterMap, LogicalResource};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccessEndpoint {
    #[serde(flatten)]
    pub logical_resource: LogicalResource,
    #[serde(rename = "apiType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<String>,
    ///URI for using the data access API
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "uriQueryFilter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri_query_filter: Option<DataFilterMap>,
}
impl std::fmt::Display for DataAccessEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DataAccessEndpoint {
    type Target = LogicalResource;
    fn deref(&self) -> &Self::Target {
        &self.logical_resource
    }
}
impl std::ops::DerefMut for DataAccessEndpoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.logical_resource
    }
}
