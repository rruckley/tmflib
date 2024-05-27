//! Work Module

use crate::{
    LIB_PATH,
    Uri,
};
use super::MOD_PATH;
use tmflib_derive::HasId;
use serde::{Deserialize,Serialize};

/// Work
#[derive(Clone,Debug,Default,Deserialize,HasId,Serialize)]
pub struct Work {
    r#type: Option<String>,
    base_type: Option<String>,
    schema_location: Option<String>,
    href: Option<Uri>,
    id: Option<String>,   
}