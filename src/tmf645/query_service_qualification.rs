//! Query Service Qualification

use serde::{Deserialize, Serialize};

// URL Path components
use super::TMF_MODULE;
use crate::TMF_VERSION;
/// TMF645 Service Qualification Module
pub const CLASS_PATH: &str = "queryServiceQualification";

use crate::{HasDescription, HasId, IsAddressable, Uri};

use tmflib_derive::{HasDescription, HasId};

use super::TaskStateType;

/// Query Service Qualification
#[derive(Clone, Debug, Default, HasId, HasDescription, Deserialize, Serialize)]
pub struct QueryServiceQualification {
    /// Unique Id
    pub id: Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Description
    pub description: Option<String>,
    /// Status
    pub state: Option<TaskStateType>,
}

impl IsAddressable for QueryServiceQualification {
    fn get_objects() -> Vec<&'static str> {
        super::get_objects()
    }
}
