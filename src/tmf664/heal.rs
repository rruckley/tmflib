//! Heal Module
//! # Description
//! Task Resource used to request healing of a Resource Function

use serde::{Deserialize, Serialize};

use crate::{DateTime, HasId, HasName, Uri, LIB_PATH};
use tmflib_derive::{HasId, HasName};

use super::{TaskStateType, MOD_PATH};

const CLASS_PATH: &str = "heal";

/// Heal Resource Function
#[derive(Clone, Debug, Default, HasId, HasName, Deserialize, Serialize)]
pub struct Heal {
    /// Unique Id
    pub id: Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Name
    pub name: Option<String>,
    // Referenced fields
    /// Tracks the lifecycle status of the migrate request.
    pub state: Option<TaskStateType>,
    /// Reason why healing is being requested.
    pub cause: Option<String>,
    /// Indicates the degree of healing required.
    pub degree_of_healing: Option<String>,
    /// Exact action to be taken as part of the heal process or a pointer to a script to be run.
    pub heal_action: Option<String>,
    /// The time when the heal action needs to commence. This allows a delay to be added.
    pub start_time: Option<DateTime>,
}
