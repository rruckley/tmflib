//! Resource Candidate Module


use serde::{Deserialize, Serialize};

const CLASS_PATH : &str = "resourceCandidate";

use super::MOD_PATH;
use crate::{LIB_PATH, HasId, CreateTMF, HasLastUpdate, CreateTMFWithTime};
use tmflib_derive::HasId;

/// Resource Candidate (Catalog Entry)
#[derive(Clone, Debug, Default, Deserialize, HasId, Serialize)]
pub struct ResourceCandidate {
    id: Option<String>,
    href: Option<String>,
    last_update: Option<String>,
    name: String,
    description: Option<String>,
}

impl ResourceCandidate {
    /// Create a new ResourceCandidate instance
    pub fn new(name : impl Into<String>) -> ResourceCandidate {
        let mut rc = ResourceCandidate::create_with_time();
        rc.name = name.into();
        rc
    }

    /// Set the description on this resource candidate
    pub fn description(mut self, description: &str) -> ResourceCandidate {
        self.description = Some(description.to_owned());
        self
    }
}

impl HasLastUpdate for ResourceCandidate {
    fn set_last_update(&mut self, time : impl Into<String>) {
        self.last_update = Some(time.into());
    }
}

impl CreateTMFWithTime<ResourceCandidate> for ResourceCandidate {}

/// Resource Candidate Reference
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResourceCandidateRef {
    id: String,
    href: String,
    name: String,
}

impl From<ResourceCandidate> for ResourceCandidateRef {
    fn from(value: ResourceCandidate) -> Self {
        ResourceCandidateRef { 
            id: value.id.as_ref().unwrap().clone(), 
            href: value.href.as_ref().unwrap().clone(), 
            name: value.name.clone(),
        }
    }
}