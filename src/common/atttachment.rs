//! Attachment Module
//! 
//! 

pub enum AttachmentType {
    InLine,
    External,
}

/// Attachment Reference or Value
pub struct AttachmentRefOrValue {
    pub id  : Option<String>,
    pub href: Option<String>,
    pub attachment_type : Option<AttachmentType>,
    pub content : Option<String>,
}