//! Check Service Qualification Module

use serde::{Deserialize,Serialize};

use crate::{
    HasId,
    HasDescription,
    LIB_PATH,
    Uri
};

use tmflib_derive::{
    HasId,
    HasDescription,
};

const CLASS_PATH : &str = "checkServiceQualification";
use super::MOD_PATH;

/// Check Service Qualification
#[derive(Clone,Debug,Default,HasId,HasDescription,Deserialize,Serialize)]
pub struct CheckServiceQualificaitonItem {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP URI
    pub href: Option<Uri>,
    /// Description
    pub description : Option<String>,
}

/// Check Service Qualification 
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct CheckServiceQualification {
    /// Service Qualification Items
    pub service_qualification_item : Option<Vec<CheckServiceQualificaitonItem>>,
}