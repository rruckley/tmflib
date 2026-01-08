//! Query Service Qualification

use serde::{Deserialize, Serialize};

use crate::{HasDescription, HasId, Uri};

use tmflib_derive::{HasDescription, HasId};

const CLASS_PATH: &str = "queryServiceQualification";
use super::{TaskStateType, MOD_PATH};

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
