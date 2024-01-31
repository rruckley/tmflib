//! Agreement Specification Module

use serde::{Deserialize,Serialize};
use crate::{LIB_PATH,HasId,HasLastUpdate,HasName,CreateTMF,CreateTMFWithTime,HasValidity,TimePeriod, DateTime};
use tmflib_derive::{HasId,HasLastUpdate,HasName,HasValidity};

use super::MOD_PATH;
const CLASS_PATH : &str = "specification";

/// Agreement Specification
#[derive(Clone,Debug,Default,Deserialize,HasId,HasName,HasLastUpdate,HasValidity,Serialize)]
pub struct AgreementSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_bundle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}

/// Reference to external specification
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