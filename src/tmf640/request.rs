use super::HeaderItem;
use serde::{Deserialize, Serialize};
///A response to a request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Request {
    ///The body of the request. For example for an HTTP request might contain content of a form .
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    ///Items included in the header of the request. For example for an HTTP request might contain requested locale, basic authentication.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<HeaderItem>>,
    ///The protocol of the request, e.g. http
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    ///The target of the request, e.g. a URL for an HTTP request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}
impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
