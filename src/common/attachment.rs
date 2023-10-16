//! Attachment Module
//!
//!
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::MOD_PATH;
use crate::LIB_PATH;

const ATTACH_PATH: &str = "attachment";

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
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
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
    /// URL where the content is stored for the external attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Size of the attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<AttachmentSize>,
    /// How long is this attachment valid for?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<String>,
}

impl AttachmentRefOrValue {
    /// Create a new attachment object
    pub fn new() -> AttachmentRefOrValue {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}", LIB_PATH, MOD_PATH, ATTACH_PATH, id);
        AttachmentRefOrValue {
            id: Some(id),
            href: Some(href),
            attachment_type: None,
            content: None,
            description: None,
            mime_type: None,
            url: None,
            size: None,
            valid_for: None,
        }
    }
}
