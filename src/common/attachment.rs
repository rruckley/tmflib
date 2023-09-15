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
    pub description : Option<String>,
    pub mime_type : Option<String>,
    pub url : Option<String>,
    pub size : u64,
    pub valid_for : Option<String>,
}