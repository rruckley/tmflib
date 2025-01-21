//! Trouble Ticket Module

use serde::{Deserialize, Serialize};
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
use crate::{
    DateTime,
    HasName,
    HasDescription,
    HasId, 
    HasLastUpdate, 
    HasNote, 
    HasRelatedParty,
    HasAttachment,
    Uri,
    // vec_insert,
    LIB_PATH,
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