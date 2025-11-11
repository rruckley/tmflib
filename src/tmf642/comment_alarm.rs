use serde::{Serialize, Deserialize};
use super::{AlarmRefOrValue, Comment, Entity};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommentAlarm {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Alarm patterns to match target alarms. An alarm will match if all of the sttributes in any of the patterns compare equal to those attributes of the alarm.
    #[serde(rename = "alarmPattern")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alarm_pattern: Vec<AlarmRefOrValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
    ///The successfully commented alarms
    #[serde(rename = "commentedAlarm")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commented_alarm: Vec<AlarmRefOrValue>,
    ///A reference to the task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///The identifier of the task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Current state of the operation task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for CommentAlarm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for CommentAlarm {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for CommentAlarm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
