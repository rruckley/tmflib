//! Migrate Module

use serde::{Deserialize, Serialize};

use crate::{HasId,HasName, LIB_PATH,Uri};
use crate::common::related_place::PlaceRef;
use tmflib_derive::{HasId,HasName};


use super::{TaskStateType, MOD_PATH};

const CLASS_PATH : &str = "migrate";
use super::resource_function::ResourceFunctionRef;

/// External connection points of the resource function. These are the service access points
/// (SAP) where inputs and outputs of the function are available.
#[derive(Clone,Debug,Default,HasId,HasName,Deserialize,Serialize)]
pub struct ConnectionPointRef {
    /// Unique Id
    id : Option<String>,
    /// HTTP Uri
    href: Option<Uri>,
    /// Name
    name : Option<String>,
    /// Version
    version: Option<String>,
}

/// Migrate a Resource Function
#[derive(Clone,Debug,Default,HasId,HasName,Deserialize,Serialize)]
pub struct Migrate {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Migration Task Name
    pub name : Option<String>,
    /// SubState required before migration is carried out.
    pub admin_state_modification : Option<String>,
    /// Reason why migration is being requested.
    pub cause : Option<String>,
    /// In what mode is the migrate operation to be performed
    pub completion_mode : Option<String>,
    /// Priority
    pub priority : Option<u16>,
    /// The time when the migration needs to commence. This allows a delay to be added.
    pub start_time : Option<String>,
    /// Tracks the lifecycle status of the migrate request.
    pub state : Option<TaskStateType>,

    // Referenced modules
    /// Resource Function
    resource_function : ResourceFunctionRef,
    /// Place Reference
    place : Option<PlaceRef>,
}

impl Migrate {

    /// Create a new migrate task
    pub fn new(name : impl Into<String>) -> Migrate {
        Migrate {
            ..Migrate::create()
        }.name(name)
    }

    /// Builder function to set place on create
    pub fn place(mut self, place : PlaceRef) -> Self {
        self.place = Some(place);
        self
    }
}