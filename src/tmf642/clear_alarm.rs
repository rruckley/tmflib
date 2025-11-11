use serde::{Serialize, Deserialize};
use super::{AlarmRefOrValue, Entity};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClearAlarm {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Time of the alarm clearing
    #[serde(rename = "alarmClearedTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_cleared_time: Option<chrono::DateTime<chrono::Utc>>,
    ///Alarm patterns to match target alarms. An alarm will match if all of the sttributes in any of the patterns compare equal to those attributes of the alarm.
    #[serde(rename = "alarmPattern")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alarm_pattern: Vec<AlarmRefOrValue>,
    ///Name of the clearing system
    #[serde(rename = "clearSystemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clear_system_id: Option<String>,
    ///Name of the clearing user
    #[serde(rename = "clearUserId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clear_user_id: Option<String>,
    ///The successfully cleared alarms
    #[serde(rename = "clearedAlarm")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cleared_alarm: Vec<AlarmRefOrValue>,
    ///Current state of the operation task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for ClearAlarm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ClearAlarm {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for ClearAlarm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
