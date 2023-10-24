//! Resource Candidate Module


use serde::{Deserialize, Serialize};

const RC_PATH : &str = "candidate";

use super::MOD_PATH;
use crate::{LIB_PATH, HasId, CreateTMF, HasLastUpdate, CreateTMFWithTime};

/// Resource Candidate (Catalog Entry)
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ResourceCandidate {
    id: Option<String>,
    href: Option<String>,
    last_update: Option<String>,
    name: String,
    description: Option<String>,
}

impl ResourceCandidate {
    /// Create a new ResourceCandidate instance
    pub fn new(name : String) -> ResourceCandidate {
        let mut rc = ResourceCandidate::create_with_time();
        rc.name = name;
        rc
    }

    /// Set the description on this resource candidate
    pub fn description(mut self, description: &str) -> ResourceCandidate {
        self.description = Some(description.to_owned());
        self
    }
}

impl HasId for ResourceCandidate {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,RC_PATH,self.get_id());
        self.href = Some(href);    
    }   
    fn generate_id(&mut self) {
        let id = ResourceCandidate::get_uuid();
        self.id = Some(id);
        self.generate_href();
    } 
    fn get_href(&self) -> String {
        self.href.as_ref().unwrap().clone()
    }
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap().clone()    
    }
    fn get_class() -> String {
        RC_PATH.to_owned()
    }
}

impl CreateTMF<ResourceCandidate> for ResourceCandidate {}

impl HasLastUpdate for ResourceCandidate {
    fn set_last_update(&mut self, time : String) {
        self.last_update = Some(time);
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