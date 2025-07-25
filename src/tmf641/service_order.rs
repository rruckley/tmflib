use serde::{Deserialize, Serialize};

// URL Path components
use crate::LIB_PATH;
use super::MOD_PATH;
use crate::{HasId,HasNote,HasRelatedParty,HasDescription, DateTime};
use tmflib_derive::{HasId,HasNote,HasRelatedParty,HasDescription};
use crate::common::note::Note;
use crate::common::tmf_error::TMFError;
use super::service_order_item::ServiceOrderItem;
use crate::common::related_party::RelatedParty;

const CLASS_PATH: &str = "serviceOrder";

/// Service Order Status
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ServiceOrderStateType {
    /// Acknowledged
    #[default]
    Acknowledged,
    /// Rejected
    Rejected,
    /// Pending
    Pending,
    /// Held
    Held,
    /// InProgress
    InProgress,
    /// Cancelled
    Cancelled,
    /// Completed
    Completed,
    /// Failed
    Failed,
    /// Partial
    Partial,
    /// Assessing Cancellation
    AccessingCancellation,
    /// Pending Cancellation
    PendingCancellation,
}

/// Service Order Object
#[derive(Clone, Debug, Default, Deserialize, HasId, HasNote, HasRelatedParty, HasDescription, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceOrder {
    /// Cancellation Date
    pub cancellation_date: Option<DateTime>,
    /// Cancellation Reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    /// Order Category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category : Option<String>,
    /// Completion Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<DateTime>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Expected Completion Date
    pub expected_completion_date: Option<DateTime>,
    /// External Id
    pub external_id : Option<String>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// URi
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Notification Contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_contact: Option<String>,
    /// Order Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_date: Option<DateTime>,
    /// Order Priority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    /// Requested Completion Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_completion_date: Option<DateTime>,
    /// Requested Start Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_start_date: Option<DateTime>,
    /// Start Date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime>,
    /// Order Status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ServiceOrderStateType>,
    /// Order Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,
    /// Service Order Items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_order_item: Option<Vec<ServiceOrderItem>>,
    /// Related Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party : Option<Vec<RelatedParty>>,
}

impl ServiceOrder {
    /// Create a new service order object
    pub fn new() -> ServiceOrder {
        let mut so = ServiceOrder::create();
        so.note = Some(vec![]);
        so.related_party = Some(vec![]);
        so
    }

    /// Safely add a new [ServiceOrderItem] to this ServiceOrder
    pub fn add_item(&mut self, item: ServiceOrderItem) {
        match self.service_order_item.as_mut() {
            Some(v) => v.push(item),
            None => self.service_order_item = Some(vec![item]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SERVICEORDERSTATE_JSON : &str = "\"Acknowledged\"";
    const SERVICEORDER_JSON : &str = "{
        \"description\" : \"Description\"
    }";
    const NOTE_TEXT : &str = "A Note";
    #[test]
    fn test_serviceorderstate_deserialize() {
        let serviceorderstate : ServiceOrderStateType = serde_json::from_str(SERVICEORDERSTATE_JSON).unwrap();

        assert_eq!(serviceorderstate,ServiceOrderStateType::Acknowledged);
    }

    #[test]
    fn test_serviceorder_deserialize() {
        let serviceorder : ServiceOrder = serde_json::from_str(SERVICEORDER_JSON).unwrap();

        assert_eq!(serviceorder.description.is_some(),true);
        assert_eq!(serviceorder.description.unwrap().as_str(),"Description");
    }

    #[test]
    fn test_serviceorder_hasnote() {
        let mut serviceorder = ServiceOrder::new();
        let note1 = Note::new(NOTE_TEXT);
        let note2 = Note::new(NOTE_TEXT);

        serviceorder.add_note(note1);
        serviceorder.add_note(note2);

        assert_eq!(serviceorder.note.is_some(),true);
        assert_eq!(serviceorder.note.unwrap().len(),2);
    }
}
