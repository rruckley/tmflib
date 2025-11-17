use serde::{Serialize, Deserialize};
use super::{AlarmRefOrValue};
use crate::common::entity::Entity;
use crate::{
    DateTime,
};

/// UnAckAlarm defines an unacknowledge alarm operation task for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnAckAlarm {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Name of the unacknowledging system
    #[serde(rename = "ackSystemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_system_id: Option<String>,
    ///Time of the unacknowledgement
    #[serde(rename = "ackTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_time: Option<DateTime>,
    ///Name of the unacknowledging user
    #[serde(rename = "ackUserId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ack_user_id: Option<String>,
    ///Alarm patterns to match target alarms. An alarm will match if all of the sttributes in any of the patterns compare equal to those attributes of the alarm.
    #[serde(rename = "alarmPattern")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alarm_pattern: Vec<AlarmRefOrValue>,
    ///A reference to the task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///The identifier of the task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Current state of the operation task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    ///The successfully unacknowledged alarms
    #[serde(rename = "unAckedAlarm")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub un_acked_alarm: Vec<AlarmRefOrValue>,
}
impl std::fmt::Display for UnAckAlarm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for UnAckAlarm {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for UnAckAlarm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
