//! Work Module

use crate::{
    LIB_PATH,
    HasId,
    CreateTMF,
    Uri,
};
use super::MOD_PATH;
use tmflib_derive::HasId;
use serde::{Deserialize,Serialize};

const CLASS_PATH : &str = "work";

/// Work
#[derive(Clone,Debug,Default,Deserialize,HasId,Serialize)]
pub struct Work {
    r#type: Option<String>,
    base_type: Option<String>,
    schema_location: Option<String>,
    href: Option<Uri>,
    id: Option<String>,
}