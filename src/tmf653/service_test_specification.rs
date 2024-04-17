//! Service Test Specification Module

use crate::{HasId,HasName, CreateTMF,Uri,LIB_PATH};
use tmflib_derive::{HasId,HasName};

use serde::{Deserialize,Serialize};

use super::MOD_PATH;
const CLASS_PATH: &str = "specification";

/// Service Test Specification
#[derive(Clone,Debug,Default,Deserialize,HasId,HasName,Serialize)]
pub struct ServiceTestSpecification {
    /// Unique Identifier
    pub id: Option<String>,
    /// HREF to specification
    pub href: Option<Uri>,
    /// Name
    pub name: Option<String>,
}