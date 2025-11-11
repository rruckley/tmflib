use serde::{Serialize, Deserialize};
use super::{AlarmRefOrValue, Entity};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AckAlarm {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Name of the acknowledging system
    #[serde(rename = "ackSystemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_system_id: Option<String>,
    ///Time of the acknowledgement
    #[serde(rename = "ackTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_time: Option<chrono::DateTime<chrono::Utc>>,
    ///Name of the acknowledging user
    #[serde(rename = "ackUserId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_user_id: Option<String>,
    ///The successfully acknowledged alarms
    #[serde(rename = "ackedAlarm")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acked_alarm: Vec<AlarmRefOrValue>,
    ///Alarm patterns to match target alarms. An alarm will match if all of the attributes in any of the patterns compare equal to those attributes of the alarm.
    #[serde(rename = "alarmPattern")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alarm_pattern: Vec<AlarmRefOrValue>,
    ///Current state of the operation task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for AckAlarm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for AckAlarm {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for AckAlarm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
