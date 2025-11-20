use serde::{Serialize, Deserialize};
use super::{DataFilterMapFvo, LogicalResourceFvo};

///Data Access Endpoint Full Value Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccessEndpointFvo {
    ///Base Logical Resource Full Value Object
    #[serde(flatten)]
    pub logical_resource_fvo: LogicalResourceFvo,
    ///The type of API (e.g., REST, SOAP)
    #[serde(rename = "apiType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<String>,
    ///URI for using the data access API
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "uriQueryFilter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri_query_filter: Option<DataFilterMapFvo>,
}
impl std::fmt::Display for DataAccessEndpointFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DataAccessEndpointFvo {
    type Target = LogicalResourceFvo;
    fn deref(&self) -> &Self::Target {
        &self.logical_resource_fvo
    }
}
impl std::ops::DerefMut for DataAccessEndpointFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.logical_resource_fvo
    }
}
