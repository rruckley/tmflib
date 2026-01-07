use serde::{Serialize, Deserialize};
use super::MonitorStateChangeEventPayload;
///The notification data structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitorStateChangeEvent {
    ///The correlation id for this event.
    #[serde(rename = "correlationId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    ///An explnatory of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The domain of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    ///The event data structure
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<MonitorStateChangeEventPayload>,
    ///The identifier of the notification.
    #[serde(rename = "eventId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    ///Time of the event occurrence.
    #[serde(rename = "eventTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<crate::DateTime>,
    ///The type of the notification.
    #[serde(rename = "eventType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    ///A priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    ///The time the event occured.
    #[serde(rename = "timeOcurred")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_ocurred: Option<crate::DateTime>,
    ///The title of the event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl std::fmt::Display for MonitorStateChangeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
