use super::HeaderItem;
use serde::{Deserialize, Serialize};
///A response to a request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Response {
    ///The body of the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    ///Items included in the header of the response. For example for an HTTP response might contain negotiated locale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<HeaderItem>>,
    ///The status of the response. For example for an HTTP response would be codes such as 200, 400, etc.
    #[serde(rename = "statusCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}
impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
