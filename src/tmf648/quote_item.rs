//! Quote Item Module

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::attachment::AttachmentRefOrValue;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuoteItem {
    id: String,
    action: Option<String>,
    quantity: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    quote_item: Option<Vec<QuoteItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<Vec<AttachmentRefOrValue>>,
}

impl QuoteItem {
    pub fn new() -> QuoteItem {
        let id = Uuid::new_v4().to_string();
        QuoteItem {
            id,
            action: None,
            quantity: 1,
            quote_item: None,
            attachment: None,
        }
    }
}
