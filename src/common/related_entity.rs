//! Related Entity Module

use serde::{Deserialize,Serialize};

use crate::Uri;
use crate::HasName;

/// Reference to another TMF schema
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct RelatedEntity {
    /// Referenced Name
    pub name : String,
    /// Referenced Id
    pub id : String,
    /// Referenced HREF
    pub href : Uri,
    /// Referenced Role
    pub role : Option<String>,
    /// Referred Type
    pub referred_type: String,
}

/// Generate a related entity from any TMF object that has implemented HasName trait
impl<T : HasName> From<T> for RelatedEntity {
    fn from (value: T) -> Self {
        RelatedEntity {
            name : value.get_name(),
            id : value.get_id(),
            href: value.get_href(),
            referred_type: T::get_class(),
            ..Default::default()
        }
    }
}