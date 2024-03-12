//! Communication Message Module

use crate::{CreateTMF, DateTime, HasId, HasName, Uri, LIB_PATH};
use serde::{Deserialize, Serialize};
use tmflib_derive::HasId;
use crate::common::attachment::AttachmentRefOrValue;
use crate::common::related_party::RelatedParty;
use crate::tmf632::individual::Individual;

use super::MOD_PATH;
const CLASS_PATH : &str = "message";

/// Recipient of this communication message
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct Receiver {
    name: String,
    id: String,
    email: Option<String>,
    /// Related party for receiver
    pub party: Option<RelatedParty>,
}

impl From<&Individual> for Receiver {
    fn from(value: &Individual) -> Self {
        Receiver {
            id: value.get_id(),
            name: value.get_name(),
            email: value.get_email(),
            party : Some(RelatedParty::from(value)),
        }
    }
}

/// Sending of this communication message
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct Sender {
    id: String,
    name: String,
    /// Related party for sender
    pub party: Option<RelatedParty>,
}

impl From<&Individual> for Sender {
    fn from(value: &Individual) -> Self {
        Sender {
            id: value.get_id(),
            name: value.get_name(),
            party: Some(RelatedParty::from(value)),
        }
    }
}

/// Message Status
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub enum CommunicationMessageStateType {
    /// Message created
    #[default]
    Initial,
    /// Delivery is in Progress
    InProgress,
    /// Delivery completed
    Completed,
}

/// Communication Message
#[derive(Clone,Default,Debug,Deserialize,HasId,Serialize)]
pub struct CommunicationMessage {
    content : String,
    /// Message Description
    pub description : Option<String>,
    /// URI for communication message
    pub href: Option<Uri>,
    /// Unique Id for message
    pub id : Option<String>,
    log_flag: bool,
    message_type: String,
    priority: String,
    scheduled_send_time: DateTime,
    send_time: DateTime,
    send_time_complete: DateTime,
    state: CommunicationMessageStateType,
    subject: Option<String>,
    try_times : u32,
    // Referenced structures
    attachment : Vec<AttachmentRefOrValue>,
    /// Reciever(s)
    pub receiver: Vec<Receiver>,
    /// Sender
    pub sender: Option<Sender>,
}

impl CommunicationMessage {
    /// Create a new basic message
    pub fn new(content : impl Into<String>) -> CommunicationMessage {
        CommunicationMessage::create()
            .content(content)
    }

    /// Create an email style message
    pub fn email(subject: impl Into<String>, content: impl Into<String>) -> CommunicationMessage {
        CommunicationMessage::new(content)
            .subject(subject)
            .message_type("email")
    }

    /// Set content of message
    pub fn content(mut self, content : impl Into<String>) -> CommunicationMessage {
        self.content = content.into();
        self
    }

    /// Add subject to message
    pub fn subject(mut self, subject : impl Into<String>) -> CommunicationMessage {
        self.subject = Some(subject.into());
        self
    }

    /// Set message description
    pub fn description(mut self, description : impl Into<String>) -> CommunicationMessage {
        self.description = Some(description.into());
        self
    }

    /// Set type of message 
    pub fn message_type(mut self,msg_type : impl Into<String>) -> CommunicationMessage {
        self.message_type = msg_type.into();
        self
    }

    /// Set the Sender for this message
    pub fn from(mut self, sender : &Individual) -> CommunicationMessage {
        self.sender = Some(Sender::from(sender));
        self
    }

    /// Set the receivers for this message
    pub fn to(mut self, recievers: Vec<&Individual>) -> CommunicationMessage {
        recievers.into_iter().for_each(|i| {
            self.receiver.push(Receiver::from(i));
        });
        self
    }
}

#[cfg(test)]
mod test {
    use super::CommunicationMessage;

    const MSG : &str = "AMessage";
    const SUB : &str = "ASubject";
    const DSC : &str = "ADescription";

    #[test]
    fn test_message_new() {
        let msg = CommunicationMessage::new(MSG);

        assert_eq!(msg.content,MSG);
    }

    #[test]
    fn test_message_subject() {
        let msg = CommunicationMessage::new(MSG)
            .subject(SUB);
    
        assert_eq!(msg.subject.unwrap(),SUB);
    }

    #[test]
    fn test_message_description() {
        let msg = CommunicationMessage::new(MSG)
            .description(DSC);

        assert_eq!(msg.description.unwrap(),DSC.to_string());
    }

    #[test]
    fn test_message_email() {
        let email = CommunicationMessage::email(SUB,MSG);

        assert_eq!(email.content,MSG);
        assert_eq!(email.subject.unwrap(),SUB);
        assert_eq!(email.message_type,"email".to_string());
    }
}