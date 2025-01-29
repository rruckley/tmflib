//! Trouble Ticket Module

use serde::{Deserialize, Serialize};
use chrono::Utc;
use tmflib_derive::{
    HasId,
    HasName,
    HasDescription,
    HasNote,
    HasRelatedParty,
    HasLastUpdate,
    HasAttachment,
};

use crate::common::attachment::AttachmentRefOrValue;
use crate::common::related_party::RelatedParty;
use crate::common::note::Note;
use crate::common::event::{Event,EventPayload};
use crate::{
    DateTime, HasAttachment, HasDescription, HasId, HasLastUpdate, HasName, HasNote, HasRelatedParty, TMFEvent, Uri, LIB_PATH
};

// URL Path components
use super::MOD_PATH;

const CLASS_PATH : &str = "troubleTicket";

/// Trouble Ticket
#[derive(Clone, Debug, Default, Deserialize, HasAttachment, HasId, HasName, HasDescription, HasNote, HasLastUpdate, HasRelatedParty, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TroubleTicket {
    #[serde(skip_serializing_if = "Option::is_none")]
    id : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    // Referenced fields
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<Vec<Note>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related_party: Option<Vec<RelatedParty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<Vec<AttachmentRefOrValue>>,
}

impl TroubleTicket {
    /// Create a new trouble ticket
    pub fn new(name : impl Into<String>) -> TroubleTicket {
        TroubleTicket {
            name: Some(name.into()),
            ..TroubleTicket::create_with_time()
        }
    }
}

/// Trouble Ticket Event Type
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub enum TroubleTicketEventType {
    /// Ticket Created
    #[default]
    TroubleTicketCreateEvent,
    /// Ticket Updated
    TroubleTicketAttributeValueChangeEvent,
    /// Ticket Status Change
    TroubleTicketStatusChangeEvent,
    /// Ticket Deleted
    TroubleTicketDeleteEvent,
    /// Ticket Resolved
    TroubleTicketResolvedEvent,
    /// Ticket Pending Information
    TroubleTicketInformationRequiredEvent,
}

/// Trouble Ticket Event Holder
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct TroubleTicketEvent {
    /// Impacted ticket
    pub ticket : TroubleTicket,
}

impl TMFEvent<TroubleTicketEvent> for TroubleTicket {
    fn event(&self) -> TroubleTicketEvent {
        TroubleTicketEvent {
            ticket : self.clone(),
        }    
    }
}

impl EventPayload<TroubleTicketEvent> for TroubleTicket {
    type Subject = TroubleTicket;
    type EventType = TroubleTicketEventType;

    fn to_event(&self,event_type : Self::EventType) -> crate::common::event::Event<TroubleTicketEvent,Self::EventType> {
        let desc = format!("{:?} for {} [{}]",event_type,self.get_name(),self.get_id());
        let now = Utc::now();
        let event_time = chrono::DateTime::from_timestamp(now.timestamp(),0).unwrap();
        Event {
            id: Some(self.get_id()),
            href: Some(self.get_href()),
            description: Some(desc),
            domain: Some(TroubleTicket::get_class()),
            title: Some(self.get_name()),
            time_occurred: Some(event_time.to_string()),
            event: self.event(),
            ..Default::default()
        }
    }
}

/// Trouble Ticket Relationship Type
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TroubleTicketRelationship {
    /// Linked Trouble Ticket Id
    pub id : String,
    /// Linked Trouble Ticket Href
    pub href : String,
    /// Linked Trouble Ticket Name
    pub name : String,
    /// Relationship type
    pub relationship_type: String,
}

impl TroubleTicketRelationship {
    /// Set the relationship on a TroubleTicketRelationship in builder pattern
    pub fn relationship(mut self, relationship : impl Into<String>) -> TroubleTicketRelationship {
        self.relationship_type = relationship.into();
        self
    }
}

impl From<TroubleTicket> for TroubleTicketRelationship {
    fn from(value: TroubleTicket) -> Self {
        TroubleTicketRelationship {
            id : value.get_id(),
            href: value.get_href(),
            name: value.get_name(),
            relationship_type : "dependency".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{TroubleTicket, TroubleTicketRelationship};
    use crate::{
        HasId,
        HasName
    };

    const TICKET_NAME:&str = "TroubleTicket";
    const TICKET_REL: &str = "TroubleTicketRelationshipType";

    #[test]
    fn test_troubleticket_new() {
        let ticket = TroubleTicket::new(TICKET_NAME);

        assert_eq!(ticket.name.is_some(),true);
        assert_eq!(ticket.get_name(),TICKET_NAME.to_string());
    }

    #[test]
    fn test_troubleticketrelationship_from() {
        let ticket = TroubleTicket::new(TICKET_NAME);

        let relationship = TroubleTicketRelationship::from(ticket.clone())
            .relationship(TICKET_REL);

        assert_eq!(ticket.get_id(),relationship.id);
        assert_eq!(ticket.get_name(),relationship.name);
        assert_eq!(relationship.relationship_type,TICKET_REL.to_string());
    }
}