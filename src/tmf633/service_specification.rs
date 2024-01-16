//! Service Specification Module

use serde::{Deserialize,Serialize};

use crate::{HasId,HasName,HasLastUpdate,CreateTMF,CreateTMFWithTime};
use tmflib_derive::{HasId,HasName,HasLastUpdate};

use super::MOD_PATH;
use crate::LIB_PATH;

const CLASS_PATH : &str = "service";

use super::characteristic_specification::CharacteristicSpecification;

/// Service Specification
#[derive(Clone,Default,Debug,Deserialize, HasId, HasName, HasLastUpdate, Serialize)]
pub struct ServiceSpecification {
    /// Unique Id
    pub id : Option<String>,
    /// HREF 
    pub href : Option<String>,
    /// Name
    pub name : Option<String>,
    /// Last Update
    pub last_update : Option<String>,
    /// Characteristics
    pub spec_characteristics: Option<Vec<CharacteristicSpecification>>,
}