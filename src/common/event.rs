//! Asynchronous Events
//! 
use serde::{Deserialize, Serialize};
use crate::{
    HasId,
    TMFEvent
};

/// Generic Event structure, will be linked into event specific payloads.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event<T,U> {
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

impl<T,U> Event<T,U> {
    /// Set the field path for a attribute change event
    pub fn path(mut self, path : impl Into<String>) -> Event<T,U> {
        self.field_path = Some(path.into());
        self
    }
}

/// Trait for types that can generate an event
pub trait EventPayload<T> {
    /// Object the event pertains to
    type Subject : HasId + TMFEvent<T>;
    /// Type of event generated
    type EventType;
    /// Convert the item into an event
    fn to_event(&self,event_type : Self::EventType) -> Event<T,Self::EventType>;
}

#[cfg(test)]
mod test {
    #[cfg(all(feature = "tmf632", feature = "build-V4"))]
    use crate::tmf632::organization_v4::Organization;
    #[cfg(all(feature = "tmf632", feature = "build-V5"))]
    use crate::tmf632::organization_v5::Organization;
    use crate::tmf629::customer::{Customer,CustomerEventType};
    use super::EventPayload;
    use super::Event;
    
    const EVENT_JSON : &str = "{
        \"eventType\" : \"CustomerCreateEvent\",
        \"eventId\" : \"E123\",
        \"eventTime\" : \"2024-01-01T13:00:00Z\",
        \"event\" : {
            \"relatedParty\" : []
        }
    }";

    const PATH : &str = "status";
    #[test]
    fn test_event_path() {
        let org = Organization::new("An Organization");
        let cust = Customer::from(&org);
    
        let event = cust.to_event(CustomerEventType::CustomerCreateEvent)
            .path(PATH);

        assert_eq!(event.field_path.unwrap(),PATH.to_string());
    }

    #[test]
    fn test_event_deserialize() {
        let event : Event<Customer,CustomerEventType> = serde_json::from_str(EVENT_JSON)
            .expect("Could not parse EVENT_JSON");

        assert_eq!(event.event_id.as_str(),"E123");
    }
}