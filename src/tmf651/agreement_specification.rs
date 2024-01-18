//! Agreement Specification Module

use serde::{Deserialize,Serialize};
use crate::{LIB_PATH,HasId,HasLastUpdate,HasName,CreateTMF,CreateTMFWithTime,TimePeriod};
use tmflib_derive::{HasId,HasLastUpdate,HasName};

use super::MOD_PATH;
const CLASS_PATH : &str = "specification";

/// Agreement Specification
#[derive(Clone,Debug,Default,Deserialize,HasId,HasName,HasLastUpdate,Serialize)]
pub struct AgreementSpecification {
    description: Option<String>,
    href: Option<String>,
    id: Option<String>,
    is_bundle: Option<bool>,
    last_update: Option<String>,
    lifecycle_status: Option<String>,
    name: Option<String>,
    valid_for: Option<TimePeriod>,
    version: Option<String>,
}

#[derive(Clone,Default,Debug, Deserialize, Serialize)]
pub struct AgreementSpecificationRef {
    description : String,
    href: String,
    id: String,
    name: String,
}

impl From<AgreementSpecification> for AgreementSpecificationRef {
    fn from(value: AgreementSpecification) -> Self {
        AgreementSpecificationRef {
            description: value.description.as_ref().unwrap().clone(),
            id : value.get_id(),
            href: value.get_href(),
            name: value.get_name(),

        }
    }
}