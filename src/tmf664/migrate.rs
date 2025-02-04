//! Migrate Module

use serde::{Deserialize, Serialize};

use crate::{HasId,HasName, LIB_PATH,Uri};
use tmflib_derive::{HasId,HasName};


use super::MOD_PATH;

const CLASS_PATH : &str = "migrate";
use super::resource_function::ResourceFunctionRef;

/// Migrate a Resource Function
#[derive(Clone,Debug,Default,HasId,HasName,Deserialize,Serialize)]
pub struct Migrate {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Migration Task Name
    pub name : Option<String>,
    // Referenced modules
    /// Resource Function
    resource_function : ResourceFunctionRef,
}