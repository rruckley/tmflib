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

#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct AttachmentSize {
    amount : f64,
    units  : String,
}

/// Attachment Reference or Value
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct AttachmentRefOrValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id  : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_type : Option<AttachmentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url : Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size : Option<AttachmentSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for : Option<String>,
}

impl AttachmentRefOrValue {
    pub fn new() -> AttachmentRefOrValue {
        let id = Uuid::new_v4().to_string();
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,ATTACH_PATH,id);
        AttachmentRefOrValue { 
            id : Some(id), 
            href : Some(href), 
            attachment_type: None, 
            content: None, 
            description: None, 
            mime_type: None, 
            url: None, 
            size: None, 
            valid_for: None 
        }
    }
}