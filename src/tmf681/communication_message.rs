//! Communication Message Module

use crate::common::attachment::AttachmentRefOrValue;
use crate::common::related_party::RelatedParty;
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use crate::tmf632::individual_v4::Individual;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use crate::tmf632::individual_v5::Individual;
use crate::{DateTime, HasAttachment, HasId, HasName, Uri};
use serde::{Deserialize, Serialize};
use tmflib_derive::{HasAttachment, HasId};

use super::MOD_PATH;
const CLASS_PATH: &str = "message";

/// Recipient of this communication message
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
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
            party: Some(RelatedParty::from(value)),
        }
    }
}

/// Sending of this communication message
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Default, Debug, Deserialize, HasId, HasAttachment, Serialize)]
pub struct CommunicationMessage {
    content: String,
    /// Message Description
    pub description: Option<String>,
    /// URI for communication message
    pub href: Option<Uri>,
    /// Unique Id for message
    pub id: Option<String>,
    log_flag: bool,
    message_type: String,
    priority: String,
    scheduled_send_time: DateTime,
    send_time: DateTime,
    send_time_complete: DateTime,
    state: CommunicationMessageStateType,
    subject: Option<String>,
    try_times: u32,
    // Referenced structures
    attachment: Option<Vec<AttachmentRefOrValue>>,
    /// Reciever(s)
    pub receiver: Vec<Receiver>,
    /// Sender
    pub sender: Option<Sender>,
}

impl CommunicationMessage {
    /// Create a new basic message
    pub fn new(content: impl Into<String>) -> CommunicationMessage {
        CommunicationMessage::create().content(content)
    }

    /// Create an email style message
    pub fn email(subject: impl Into<String>, content: impl Into<String>) -> CommunicationMessage {
        CommunicationMessage::new(content)
            .subject(subject)
            .message_type("email")
    }

    /// Set content of message
    pub fn content(mut self, content: impl Into<String>) -> CommunicationMessage {
        self.content = content.into();
        self
    }

    /// Add subject to message
    pub fn subject(mut self, subject: impl Into<String>) -> CommunicationMessage {
        self.subject = Some(subject.into());
        self
    }

    /// Set message description
    pub fn description(mut self, description: impl Into<String>) -> CommunicationMessage {
        self.description = Some(description.into());
        self
    }

    /// Set type of message
    pub fn message_type(mut self, msg_type: impl Into<String>) -> CommunicationMessage {
        self.message_type = msg_type.into();
        self
    }

    /// Set the Sender for this message
    pub fn from(mut self, sender: &Individual) -> CommunicationMessage {
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
    #[cfg(feature = "build-V4")]
    use crate::tmf632::individual_v4::Individual;
    #[cfg(feature = "build-V5")]
    use crate::tmf632::individual_v5::Individual;
    use crate::{HasId, HasName};

    use super::{CommunicationMessage, Receiver, Sender};

    const MSG: &str = "AMessage";
    const SUB: &str = "ASubject";
    const DSC: &str = "ADescription";

    #[test]
    fn test_message_new() {
        let msg = CommunicationMessage::new(MSG);

        assert_eq!(msg.content, MSG);
    }

    #[test]
    fn test_message_subject() {
        let msg = CommunicationMessage::new(MSG).subject(SUB);

        assert_eq!(msg.subject.unwrap(), SUB);
    }

    #[test]
    fn test_message_description() {
        let msg = CommunicationMessage::new(MSG).description(DSC);

        assert_eq!(msg.description.unwrap(), DSC.to_string());
    }

    #[test]
    fn test_message_email() {
        let email = CommunicationMessage::email(SUB, MSG);

        assert_eq!(email.content, MSG);
        assert_eq!(email.subject.unwrap(), SUB);
        assert_eq!(email.message_type, "email".to_string());
    }

    #[test]
    fn test_receiver_from_individual() {
        let individual = Individual::new("An Individual");

        let rcv = Receiver::from(&individual);

        assert_eq!(individual.get_id(), rcv.id);
        assert_eq!(individual.get_name(), rcv.name);
    }

    #[test]
    fn test_sender_from_individual() {
        let individual = Individual::new("An Individual");

        let snd = Sender::from(&individual);

        assert_eq!(individual.get_id(), snd.id);
        assert_eq!(individual.get_name(), snd.name);
    }

    #[test]
    fn test_message_from() {
        let individual = Individual::new("An Individual");
        let email = CommunicationMessage::email(SUB, MSG).from(&individual);

        assert_eq!(email.sender.is_some(), true);
    }
}
