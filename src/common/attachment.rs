//! Attachment Module
//! 
//! 
use serde::{Deserialize,Serialize};
use uuid::Uuid;

use crate::LIB_PATH;
use super::MOD_PATH;

const ATTACH_PATH : &str = "attachment";

#[derive(Clone,Debug,Deserialize,Serialize)]
pub enum AttachmentType {
    InLine,
    External,
}

/// Attachment Reference or Value
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct AttachmentRefOrValue {
    pub id  : String,
    pub href: String,
    pub attachment_type : Option<AttachmentType>,
    pub content : Option<String>,
    pub description : Option<String>,
    pub mime_type : Option<String>,
    pub url : Option<String>,
    pub size : u64,
    pub valid_for : Option<String>,
}

impl AttachmentRefOrValue {
    pub fn new() -> AttachmentRefOrValue {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,ATTACH_PATH,id);
        AttachmentRefOrValue { 
            id, 
            href, 
            attachment_type: None, 
            content: None, 
            description: None, 
            mime_type: None, 
            url: None, 
            size: 0, 
            valid_for: None 
        }
    }
}