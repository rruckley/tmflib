//! Service Candidate Modules

use serde::{Deserialize, Serialize};
use std::convert::From;

use crate::{HasId,HasLastUpdate, HasName, LIB_PATH};
use tmflib_derive::{HasId, HasLastUpdate, HasName};

use super::MOD_PATH;
const CLASS_PATH : &str = "serviceCandidate";

/// Service Candidate 
#[derive(Clone, Debug, Default, Deserialize, HasId, HasLastUpdate, HasName, Serialize)]
pub struct ServiceCandidate {
    id: Option<String>,
    href: Option<String>,
    name: Option<String>,
    last_update: Option<String>,
}

impl ServiceCandidate {
    /// Create new instance of Service Candidate
    pub fn new(name : impl Into<String>) -> ServiceCandidate {
        let mut sc = ServiceCandidate::create_with_time();
        sc.name = Some(name.into());
        sc
    }
}

impl From<String> for ServiceCandidate {
    fn from(value: String) -> Self {
        ServiceCandidate::new(value)
    }
}

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
            name: value.get_name(),
        }
    }
}