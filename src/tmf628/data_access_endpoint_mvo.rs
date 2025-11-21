use serde::{Serialize, Deserialize};
use super::{DataFilterMapMvo, LogicalResourceMvo};

///Data Access Endpoint Multi Value Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccessEndpointMvo {
    ///Base Logical Resource Multi Value Object
    #[serde(flatten)]
    pub logical_resource_mvo: LogicalResourceMvo,
    ///The type of data access API (e.g. REST, SOAP)
    #[serde(rename = "apiType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<String>,
    ///URI for using the data access API
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    ///Query filter parameters for the data access API
    #[serde(rename = "uriQueryFilter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri_query_filter: Option<DataFilterMapMvo>,
}
impl std::fmt::Display for DataAccessEndpointMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for DataAccessEndpointMvo {
    type Target = LogicalResourceMvo;
    fn deref(&self) -> &Self::Target {
        &self.logical_resource_mvo
    }
}
impl std::ops::DerefMut for DataAccessEndpointMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.logical_resource_mvo
    }
}
