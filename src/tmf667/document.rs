//! Document Module

use crate::common::related_entity::RelatedEntity;
use crate::common::related_party::RelatedParty;
use crate::common::tmf_error::TMFError;
use crate::vec_insert;
use crate::{
    common::attachment::AttachmentRefOrValue, DateTime, HasDescription, HasId, HasLastUpdate,
    HasName, HasRelatedParty, Uri,
};
use serde::{Deserialize, Serialize};
use tmflib_derive::{HasDescription, HasId, HasLastUpdate, HasName, HasRelatedParty};

const CLASS_PATH: &str = "document";
use super::MOD_PATH;
const DOC_VERSION: &str = "1.0";

/// Document State
#[derive(Clone, Default, Debug, Deserialize, PartialEq, Serialize)]
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
#[derive(
    Clone,
    Default,
    Debug,
    Deserialize,
    HasId,
    HasName,
    HasLastUpdate,
    HasRelatedParty,
    HasDescription,
    Serialize,
)]
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
    /// Parties
    related_party: Option<Vec<RelatedParty>>,
    /// Related Entities
    related_entity: Option<Vec<RelatedEntity>>,
    /// Attachement
    attachment: AttachmentRefOrValue,
}

impl Document {
    /// Create a new document
    pub fn new(name: impl Into<String>) -> Document {
        let mut doc = Document::create_with_time();
        doc.name = Some(name.into());
        doc.creation_date = Some(Document::get_timestamp());
        doc.status = Some(DocumentStatusType::Created);
        doc.version = Some(DOC_VERSION.into());
        doc
    }

    /// Set the attachment for this document.
    pub fn attachment(mut self, attachment: AttachmentRefOrValue) -> Document {
        self.attachment = attachment;
        self
    }

    /// Set the document type
    /// ```
    /// use tmflib::tmf667::document::Document;
    /// let doc = Document::new("My Document")
    ///     .doc_type("PDF");
    /// ```
    pub fn doc_type(mut self, r#type: impl Into<String>) -> Document {
        self.document_type = Some(r#type.into());
        self
    }

    /// Link another TMF entity during creation
    pub fn link<T: HasName>(mut self, entity: T) -> Document {
        self.link_entity(entity);
        self
    }

    /// Link another TMF entity into this document
    pub fn link_entity<T: HasName>(&mut self, entity: T) {
        vec_insert(&mut self.related_entity, RelatedEntity::from(entity));
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
    use crate::common::attachment::AttachmentRefOrValue;
    use crate::tmf651::agreement::Agreement;

    use super::Document;
    use crate::HasName;

    const DOC_NAME: &str = "A Document";
    const DOC_TYPE: &str = "PDF";
    const DOC_STATE: &str = "\"Created\"";
    const AGREEMENT_NAME: &str = "AnAgreement";

    #[test]
    fn test_document_new() {
        let doc = Document::new(DOC_NAME);

        assert_eq!(doc.name, Some(DOC_NAME.into()));
    }

    #[test]
    fn test_document_new_type() {
        let doc = Document::new(DOC_NAME).doc_type(DOC_TYPE);

        assert_eq!(doc.document_type, Some(DOC_TYPE.into()));
    }

    #[test]
    fn test_document_new_version() {
        let doc = Document::new(DOC_NAME);

        assert_eq!(doc.version, Some(DOC_VERSION.into()));
    }

    #[test]
    fn test_document_new_status() {
        let doc = Document::new(DOC_NAME);

        assert_eq!(doc.status.unwrap(), DocumentStatusType::Created);
    }

    #[test]
    fn test_docstatustype_deserialize() {
        let docstatustype: DocumentStatusType = serde_json::from_str(DOC_STATE).unwrap();

        assert_eq!(docstatustype, DocumentStatusType::Created);
    }

    #[test]
    fn test_document_from_attachref() {
        let attachref = AttachmentRefOrValue::new();

        // Cloning attachref as from() consumes
        let doc = Document::from(attachref.clone());

        assert_eq!(doc.get_name(), attachref.get_name());
    }

    #[test]
    fn test_document_link() {
        let agreement = Agreement::new(AGREEMENT_NAME);

        let document = Document::new(AGREEMENT_NAME).link(agreement);

        assert_eq!(document.related_entity.is_some(), true);
        assert_eq!(document.related_entity.unwrap().first().is_some(), true);
    }
}
