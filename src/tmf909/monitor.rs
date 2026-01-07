use serde::{Serialize, Deserialize};
use super::{Request, Response};
///Monitoring of resources
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Monitor {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///reference to this monitor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Identifier of an instance of the monitor. Required to be unique within the resource type.  Used in URIs as the identifier for specific instances of a type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///A response to a request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    ///A response to a request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<Response>,
    ///The monitored resource href
    #[serde(rename = "sourceHref")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_href: Option<String>,
    ///The Monitor state of the resource.  InProgress, InError, Completed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
