//! Attachment Module
//!
//!
use serde::{Deserialize, Serialize};
use crate::{HasId, HasName, HasValidity, DateTime};
use tmflib_derive::{HasId, HasName, HasValidity};
use crate::tmf667::document::Document;

use crate::TimePeriod;

use super::MOD_PATH;
use crate::LIB_PATH;

const CLASS_PATH: &str = "attachment";

/// Attachment Type
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AttachmentType {
    /// Inline Attachment, i.e. inside the payload
    #[default]
    InLine,
    /// External Attachment, content is housed elsewhere
    External,
}

/// Attachment Size
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentSize {
    amount: f64,
    units: String,
}

/// Attachment Reference or Value
#[derive(Clone, Default, Debug, Deserialize, HasId, HasName, HasValidity, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentRefOrValue {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTML Reference to this object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// Where is the attachment located? Inline = payload, External = elsewhere
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<AttachmentType>,
    /// Content of the attachment for inline attachments00
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Description of the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Mime Type of the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Name of document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name : Option<String>,
    /// URL where the content is stored for the external attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Size of the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<AttachmentSize>,
    /// How long is this attachment valid for?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}

impl AttachmentRefOrValue {
    /// Create a new attachment object
    pub fn new() -> AttachmentRefOrValue {
        AttachmentRefOrValue::create()
    }
}

impl From<&Document> for AttachmentRefOrValue {
    fn from(value: &Document) -> Self {
        let validity  = match value.last_update.as_ref() {
            Some(t) => Some(TimePeriod::from(t.clone() as DateTime)),
            None => None,
        };
        AttachmentRefOrValue {
            name: Some(value.get_name()),
            id: Some(value.get_id()),
            href: Some(value.get_href()),
            description: value.description.clone(),
            valid_for : validity,
            ..Default::default()
        }
    }
}
