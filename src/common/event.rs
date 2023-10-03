//! Asynchronous Events
//! 
use serde::{Deserialize, Serialize};

/// Generic Event structure, will be linked into event specific payloads.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct Event<T,U> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    pub event_id: String,
    pub event_time: String,
    pub event_type: U,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_occurred: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub event: T,
}

/// Trait for types that can generate an event
pub trait EventPayload<T,U> {
    fn generate_event(&self,event_type : U) -> Event<T,U>;
}