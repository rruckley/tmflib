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
        ServiceCandidate {
            name: Some(name.into()),
            ..ServiceCandidate::create_with_time()
        }
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

#[cfg(test)]
mod test {
    use super::*;

    const CANDIDATE_NAME : &str = "CandidateName";
    #[test]
    fn test_servicecandidate_new() {
        let candidate = ServiceCandidate::new(CANDIDATE_NAME);

        assert_eq!(candidate.get_name().as_str(),CANDIDATE_NAME);
        assert_eq!(candidate.id.is_some(),true);
        assert_eq!(candidate.href.is_some(),true);
    }

    #[test]
    fn test_servicecandidate_from_string() {
        let candidate = ServiceCandidate::from(CANDIDATE_NAME.to_string());  

        assert_eq!(candidate.get_name().as_str(),CANDIDATE_NAME);
        assert_eq!(candidate.id.is_some(),true);
        assert_eq!(candidate.href.is_some(),true); 
    }

    #[test]
    fn test_candidateref_from_candidate() {
        let candidate = ServiceCandidate::from(CANDIDATE_NAME.to_string()); 

        let candidate_ref = ServiceCandidateRef::from(candidate.clone());

        assert_eq!(candidate.get_name(),candidate_ref.name);
        assert_eq!(candidate.get_id(),candidate_ref.id);
        assert_eq!(candidate.get_href(),candidate_ref.href);
    }
}