//! Agreement Module

use serde::{Deserialize,Serialize};
use crate::{LIB_PATH,HasId,HasName,CreateTMF};
use tmflib_derive::{HasId,HasName};

use super::MOD_PATH;
const CLASS_PATH : &str = "agreement";

/// Agreeement / Contract
#[derive(Clone,Default,Debug, Deserialize, HasId, HasName, Serialize)]
pub struct Agreement {
    /// Unique Id
    pub id : Option<String>,
    /// URI for Agreement
    pub href: Option<String>,
    /// Name of Agreement
    pub name: Option<String>,
}

impl Agreement {
    /// Create a new Agreement
    pub fn new(name : impl Into<String>) -> Agreement {
        let mut agreement = Agreement::create();
        agreement.name = Some(name.into());
        agreement
    }
}