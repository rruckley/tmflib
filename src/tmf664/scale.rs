//! Scale Module

use serde::{Deserialize, Serialize};

use crate::{HasId, LIB_PATH,HasName,Uri};
use tmflib_derive::{HasId,HasName};

use super::MOD_PATH;
use super::resource_function::ResourceFunctionRef;

const CLASS_PATH : &str = "scale";

/// Scale a Resource Function
#[derive(Clone,Debug,Default,HasId,HasName,Deserialize,Serialize)]
pub struct Scale {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Name
    pub name : Option<String>,
    // Referenced Modules
    /// Resource Function
    pub resource_function : ResourceFunctionRef,
}