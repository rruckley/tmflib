use serde::{Serialize, Deserialize};
use super::ResourceMvo;

///Logical Resource MVO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalResourceMvo {
    ///Base Resource MVO schema for use in TMForum Open-APIs - When used for in a schema it means that
    #[serde(flatten)]
    pub resource_mvo: ResourceMvo,
    ///the value of the logical resource. E.g '0746712345' for  MSISDN's
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl std::fmt::Display for LogicalResourceMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for LogicalResourceMvo {
    type Target = ResourceMvo;
    fn deref(&self) -> &Self::Target {
        &self.resource_mvo
    }
}
impl std::ops::DerefMut for LogicalResourceMvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource_mvo
    }
}
