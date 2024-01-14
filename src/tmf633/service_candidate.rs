//! Service Candidate Modules

use serde::{Deserialize, Serialize};
use std::convert::From;

use crate::{HasId,HasLastUpdate, CreateTMF, CreateTMFWithTime, LIB_PATH};
use tmflib_derive::HasId;

use super::MOD_PATH;
const CLASS_PATH : &str = "serviceCandidate";

/// Service Candidate 
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
pub struct ServiceCandidate {
    id: Option<String>,
    href: Option<String>,
    name: String,
    last_update: Option<String>,
}

impl ServiceCandidate {
    /// Create new instance of Service Candidate
    pub fn new(name : impl Into<String>) -> ServiceCandidate {
        let mut sc = ServiceCandidate::create_with_time();
        sc.name = name.into();
        sc
    }
}

impl From<String> for ServiceCandidate {
    fn from(value: String) -> Self {
        ServiceCandidate::new(value)
    }
}


impl HasLastUpdate for ServiceCandidate {
    fn set_last_update(&mut self, time : impl Into<String>) {
        self.last_update = Some(time.into());
    }
}

impl CreateTMFWithTime<ServiceCandidate> for ServiceCandidate {}

/// Service Candidate Reference
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceCandidateRef {
    id: String,
    href: String,
    name: String,
}

impl From<ServiceCandidate> for ServiceCandidateRef {
    fn from(value: ServiceCandidate) -> Self {
        ServiceCandidateRef { 
            id: value.get_id(), 
            href: value.get_href(), 
            name: value.name.clone(),
        }
    }
}