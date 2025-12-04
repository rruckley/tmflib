use super::Extensible;
use crate::DateTime;
use serde::{Deserialize, Serialize};

/// Comment defines a comment for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Comment {
    ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///Indicates the text of the comment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    ///Indicates the system identifier on which the client set the comment.
    #[serde(rename = "systemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_id: Option<String>,
    ///Indicates the time commenting the alarm
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,
    ///Indicates the user commenting the alarm.
    #[serde(rename = "userId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Comment {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for Comment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}
