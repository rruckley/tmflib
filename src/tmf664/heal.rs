//! Heal Module
//! 

use serde::{Deserialize, Serialize};

use crate::{HasId, LIB_PATH,HasName,Uri};
use tmflib_derive::{HasId,HasName};


use super::MOD_PATH;

const CLASS_PATH : &str = "heal";

/// Heal Resource Function
#[derive(Clone,Debug,Default,HasId,HasName,Deserialize,Serialize)]
pub struct Heal {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP Uri
    pub href: Option<Uri>,
    /// Name
    pub name : Option<String>,
}