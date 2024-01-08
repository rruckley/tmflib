//! Service Candidate Modules

use serde::{Deserialize, Serialize};
use std::convert::From;

use crate::{HasId,HasLastUpdate, CreateTMF, CreateTMFWithTime, LIB_PATH};

use super::MOD_PATH;
const SC_PATH : &str = "candidate";

/// Service Candidate 
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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


impl HasId for ServiceCandidate {
    fn generate_href(&mut self) {
        let href = format!("/{}/{}/{}/{}",LIB_PATH,MOD_PATH,SC_PATH,self.get_id());
        self.href = Some(href);       
    }
    fn generate_id(&mut self) {
        let id = ServiceCandidate::get_uuid();
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
        SC_PATH.to_owned()
    }
}

impl CreateTMF<ServiceCandidate> for ServiceCandidate {}

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