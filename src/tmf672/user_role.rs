//! User Role Module

use crate::{LIB_PATH,Uri,HasId};
use tmflib_derive::HasId;
use serde::{Deserialize,Serialize};

use super::MOD_PATH;

const CLASS_PATH : &str = "role";

/// Entitlement for this role
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct Entitlement {
    action : String,
    function : String,
    id : String,
}

/// User Role
#[derive(Clone,Debug,Default,Deserialize,HasId,Serialize)]
pub struct UserRole {
    href: Option<Uri>,
    id: Option<String>,
    involvement_role: String,
    // Referenced structures
    entitlement: Vec<Entitlement>,
}