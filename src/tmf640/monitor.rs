use serde::{Serialize, Deserialize};
use super::{Entity, Request, Response};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Monitor {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
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
impl std::ops::Deref for Monitor {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Monitor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
