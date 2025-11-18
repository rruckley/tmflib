use serde::{Serialize, Deserialize};
use super::Resource;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalResource {
    #[serde(flatten)]
    pub resource: Resource,
    ///the value of the logical resource. E.g '0746712345' for  MSISDN's
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl std::fmt::Display for LogicalResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for LogicalResource {
    type Target = Resource;
    fn deref(&self) -> &Self::Target {
        &self.resource
    }
}
impl std::ops::DerefMut for LogicalResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource
    }
}
