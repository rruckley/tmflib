//! Permission Module

use crate::{LIB_PATH,HasId,DateTime,TimePeriod,Uri,CreateTMF};
use tmflib_derive::HasId;
use serde::{Deserialize,Serialize};
use crate::common::related_party::RelatedParty;
use super::MOD_PATH;

const CLASS_PATH : &str = "permission";

/// User Permissions Struct
#[derive(Clone,Debug,Default,Deserialize,HasId,Serialize)]
pub struct Permission {
    creation_date: Option<DateTime>,
    description: Option<String>,
    href: Option<Uri>,
    id: Option<String>,
    valid_for: Option<TimePeriod>,
    // Referenced structures
    granter : Option<RelatedParty>,
    user: RelatedParty,
}