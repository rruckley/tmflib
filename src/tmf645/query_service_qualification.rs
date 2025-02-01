//! Query Service Qualification

use serde::{Deserialize,Serialize};

use crate::{
    HasId,
    HasDescription,
    LIB_PATH,
    Uri
};

use tmflib_derive::{
    HasId,
    HasDescription
};

const CLASS_PATH : &str = "queryServiceQualification";
use super::{
    MOD_PATH,
    TaskStateType,
};

/// Query Service Qualification
#[derive(Clone,Debug,Default,HasId,HasDescription,Deserialize,Serialize)]
pub struct QueryServiceQualification {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Description
    pub description: Option<String>,
    /// Status
    pub state : Option<TaskStateType>,
}