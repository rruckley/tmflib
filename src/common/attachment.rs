//! Attachment Module
//!
//!
use serde::{Deserialize, Serialize};
use crate::{
    HasId, 
    HasName, 
    HasValidity, 
    HasDecription,
    DateTime
};
use tmflib_derive::{HasId, HasName, HasValidity};
use crate::tmf667::document::Document;

use crate::TimePeriod;

use super::MOD_PATH;
use crate::LIB_PATH;

const CLASS_PATH: &str = "attachment";

/// Attachment Type
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
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
    /// Amount of units
    pub amount: f64,
    /// Units, e.g. bytes
    pub units: String,
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
        AttachmentRefOrValue {
            valid_for: Some(TimePeriod::default()),
            ..AttachmentRefOrValue::create()
        }
    }
}

impl HasDecription for AttachmentRefOrValue {
    fn description(mut self, description : impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    fn get_description(&self) -> String {
        match self.description.as_ref() {
            Some(d) => d.clone(),
            None => String::default(),
        }    
    }
    fn set_description(&mut self, description : impl Into<String>) -> Option<String> {
        self.description.replace(description.into())
    }
}

impl From<&Document> for AttachmentRefOrValue {
    fn from(value: &Document) -> Self {
        let validity  = value.last_update.as_ref().map(|t| TimePeriod::from(t.clone() as DateTime));
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

#[cfg(test)]
mod test {
    use crate::{tmf667::document::Document, HasName};

    use super::*;

    const ATTACH_TYPE_JSON : &str = "\"inLine\"";
    const ATTACH_SIZE : &str = "{
        \"amount\" : 123.4,
        \"units\" : \"bytes\"  
    }";
    const ATTACH_NAME: &str= "AttachmentName";

    const ATTACH_JSON : &str = "{}";
    const ATTACH_DESC : &str = "AttachmentDescription";

    #[test]
    fn test_attachment_default() {
        let attachment = AttachmentRefOrValue::new();

        assert_eq!(attachment.valid_for.is_some(),true);
    }

    #[test]
    fn test_attachment_from_document() {
        let document = Document::new("A Document");

        let attachment = AttachmentRefOrValue::from(&document);

        assert_eq!(attachment.get_name(),document.get_name());
    }

    #[test]
    fn test_attachmenttype_deserialize() {
        let attach_type : AttachmentType = serde_json::from_str(ATTACH_TYPE_JSON).expect("Could not parse test json");

        assert_eq!(attach_type,AttachmentType::InLine);
    }

    #[test]
    fn test_attachmentsize_deserialize() {
        let attach_size : AttachmentSize = serde_json::from_str(ATTACH_SIZE).expect("Could not parse test json");

        assert_eq!(attach_size.amount,123.4);
        assert_eq!(attach_size.units.as_str(),"bytes");
    }

    #[test]
    fn test_attach_deserialize() {
        let _attach : AttachmentRefOrValue = serde_json::from_str(ATTACH_JSON).expect("Could not parse attach JSON");

    }

    #[test]
    fn test_attach_hasname() {
        let mut attach = AttachmentRefOrValue::new();
        
        attach.set_name(ATTACH_NAME);

        assert_eq!(attach.get_name().as_str(),ATTACH_NAME);
    }

    #[test]
    fn test_attach_hasvalidity() {
        let mut attach = AttachmentRefOrValue::new();
        attach.set_validity(TimePeriod::period_30days());

        assert_eq!(attach.valid_for.is_some(),true);
        assert_eq!(attach.valid_for.unwrap().started(),true);
    }

    #[test]
    fn test_attach_description() {
        let attach = AttachmentRefOrValue::new()
            .description(ATTACH_DESC);

        assert_eq!(attach.description.is_some(),true);
        assert_eq!(attach.get_description().as_str(),ATTACH_DESC);
    }

    #[test]
    fn test_attach_setdescription() {
        let mut attach = AttachmentRefOrValue::new();

        attach.set_description(ATTACH_DESC);

        assert_eq!(attach.description.is_some(),true);
        assert_eq!(attach.get_description().as_str(),ATTACH_DESC);        
    }
}
