//! Document Module

use crate::{
    common::attachment::AttachmentRefOrValue, DateTime, HasId, HasLastUpdate, HasName, HasRelatedParty, Uri, LIB_PATH
};
use tmflib_derive::{HasId,HasName,HasLastUpdate,HasRelatedParty};
use crate::common::related_party::RelatedParty;
use serde::{Deserialize,Serialize};

const CLASS_PATH : &str = "document";
use super::MOD_PATH;
const DOC_VERSION : &str = "1.0";

/// Document State
#[derive(Clone,Default,Debug,Deserialize,PartialEq,Serialize)]
pub enum DocumentStatusType {
    /// Document has been created but is not yet review or approved.
    #[default]
    Created,
    /// Document has been Reviewed
    Reviewed,
    /// Document has been approved.
    Approved,
    /// Document has been published
    Published,
    /// Document has been archived.
    Archived,
    /// Document has been deleted and is no longer available.
    Deleted,
}

/// TMF667 Document
#[derive(Clone,Default,Debug,Deserialize,HasId,HasName,HasLastUpdate,HasRelatedParty,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    // HasId
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    // HasName
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// For trait HasLastUpdate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<DocumentStatusType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document_type: Option<String>,
    /// Description of document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_date: Option<DateTime>,
    // Referenced objects
    related_party: Option<Vec<RelatedParty>>,
    /// Attachement
    attachment : AttachmentRefOrValue,
}

impl Document {
    /// Create a new document
    pub fn new(name : impl Into<String>) -> Document {
        let mut doc = Document::create_with_time();
        doc.name = Some(name.into());
        doc.creation_date = Some(Document::get_timestamp());
        doc.status = Some(DocumentStatusType::Created);
        doc.version = Some(DOC_VERSION.into());
        doc
    }

    /// Set the attachment for this document.
    pub fn attachment(mut self, attachment : AttachmentRefOrValue) -> Document {
        self.attachment = attachment;
        self
    }

    /// Set the document type
    /// ```
    /// use tmflib::tmf667::document::Document;
    /// let doc = Document::new("My Document")
    ///     .doc_type("PDF");
    /// ```
    pub fn doc_type(mut self, r#type : impl Into<String>) -> Document {
        self.document_type = Some(r#type.into());
        self
    }
}

impl From<AttachmentRefOrValue> for Document {
    fn from(value: AttachmentRefOrValue) -> Self {
        let mut document = Document::create_with_time();
        document.set_name(value.get_name());
        document.description.clone_from(&value.description);
        document.status = Some(DocumentStatusType::Created);
        document.attachment = value.clone();
        document
    }
}

#[cfg(test)]
mod test {
    use super::DocumentStatusType;
    use super::DOC_VERSION;

    use super::Document;

    const DOC_NAME : &str  = "A Document";
    const DOC_TYPE : &str = "PDF";
    #[test]
    fn test_document_new() {
        let doc = Document::new(DOC_NAME);
        

        assert_eq!(doc.name,Some(DOC_NAME.into()));
    }

    #[test]
    fn test_document_new_type() {
        let doc = Document::new(DOC_NAME)
            .doc_type(DOC_TYPE);

        assert_eq!(doc.document_type,Some(DOC_TYPE.into()));    
    }

    #[test]
    fn test_document_new_version() {
        let doc = Document::new(DOC_NAME);

        assert_eq!(doc.version,Some(DOC_VERSION.into()));
    }

    #[test]
    fn test_document_new_status() {
        let doc = Document::new(DOC_NAME);

        assert_eq!(doc.status.unwrap(),DocumentStatusType::Created);
    }
}