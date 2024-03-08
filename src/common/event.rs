//! Asynchronous Events
//! 
use serde::{Deserialize, Serialize};
use crate::{HasId,HasName,TMFEvent};

/// Generic Event structure, will be linked into event specific payloads.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<T : HasId + HasName,U> {
    /// Correlation Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    /// Description of this event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Domain of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Event Id
    pub event_id: String,
    /// Event creation timestamp
    pub event_time: String,
    /// Class of the event
    pub event_type: U,
    /// Field Path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
    /// HTML Reference to the referenced object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Unique id of the referenced object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Priority of this event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// When did the event happen?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_occurred: Option<String>,
    /// Title of the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Actual event specific payload
    pub event: T,
}

/// Trait for types that can generate an event
pub trait EventPayload<T> {
    /// Object the event pertains to
    type Subject : HasId + HasName + TMFEvent<T>;
    /// Type of event generateds
    type EventType;
    /// Convert the item into an event
    fn to_event(&self,event_type : Self::EventType) -> Event<Self::Subject,Self::EventType>;
}