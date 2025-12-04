use super::AlarmRefOrValue;
use crate::{common::entity::Entity, DateTime};
use serde::{Deserialize, Serialize};

/// GroupAlarm defines a group alarm for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupAlarm {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Time of the correlation
    #[serde(rename = "alarmChangedTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alarm_changed_time: Option<DateTime>,
    ///Correlated alarms
    #[serde(rename = "correlatedAlarm")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub correlated_alarm: Vec<AlarmRefOrValue>,
    ///The successfully correlated alarms
    #[serde(rename = "groupedAlarm")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grouped_alarm: Vec<AlarmRefOrValue>,
    ///A reference to the task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///The identifier of the task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///An alarm defined by reference or value. The polymorphic attributes @type, @schemaLocation & @referredType are related to the Alarm entity and not the AlarmRefOrValue class itself
    #[serde(rename = "parentAlarm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_alarm: Option<AlarmRefOrValue>,
    ///Source system identifier
    #[serde(rename = "sourceSystemId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_system_id: Option<String>,
    ///Current state of the operation task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for GroupAlarm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for GroupAlarm {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for GroupAlarm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
