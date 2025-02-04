//! Scale Module
//! # Description
//! Task Resource used to request scealing of a Resource Function

use serde::{Deserialize, Serialize};

use crate::{HasId, LIB_PATH,HasName,Uri};
use tmflib_derive::{HasId,HasName};

use super::{MOD_PATH,TaskStateType};
use super::resource_function::ResourceFunctionRef;

const CLASS_PATH : &str = "scale";

/// Schedule Reference
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct ScheduleRef {
    /// Unique Id
    pub id : String,
    /// HTTP Uri
    pub href : Option<String>,
    /// Name
    pub name : Option<String>,
}

/// Scale a Resource Function
#[derive(Clone,Debug,Default,HasId,HasName,Deserialize,Serialize)]
pub struct Scale {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Name
    pub name : Option<String>,
    /// Scaling aspect is the dimension along which the resource function needs to be scaled. The id of the aspect is provided here.
    pub aspect_id : Option<String>,
    /// Number of scaling steps in the direction indicated by type of scale.
    pub number_of_steps: Option<u32>,
    /// Type of scaling requested.
    pub scale_type : Option<String>,
    /// Status
    pub state : Option<TaskStateType>,
    // Referenced Modules
    /// Resource Function
    pub resource_function : ResourceFunctionRef,
    /// Schedules
    pub schedule : Option<Vec<ScheduleRef>>,
}

impl Scale {
    /// Create a new instance of Scale object
    pub fn new(name : impl Into<String>) -> Scale {
        Scale {
            ..Scale::create()
        }.name(name)
    }

    // pub fn schedule(mut self, )
}