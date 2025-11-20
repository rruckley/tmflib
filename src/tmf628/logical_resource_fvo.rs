use serde::{Serialize, Deserialize};
use super::ResourceFvo;

///Logical Resource Full Value Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalResourceFvo {
    ///Base Resource Full Value Object
    #[serde(flatten)]
    pub resource_fvo: ResourceFvo,
    ///the value of the logical resource. E.g '0746712345' for  MSISDN's
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl std::fmt::Display for LogicalResourceFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for LogicalResourceFvo {
    type Target = ResourceFvo;
    fn deref(&self) -> &Self::Target {
        &self.resource_fvo
    }
}
impl std::ops::DerefMut for LogicalResourceFvo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resource_fvo
    }
}
