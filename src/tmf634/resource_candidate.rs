//! Resource Candidate Module


use serde::{Deserialize, Serialize};

const CLASS_PATH : &str = "resourceCandidate";

use super::MOD_PATH;
use crate::{LIB_PATH, HasId, HasName, HasLastUpdate};
use tmflib_derive::{HasId, HasName};

/// Resource Candidate (Catalog Entry)
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, Serialize)]
pub struct ResourceCandidate {
    id: Option<String>,
    href: Option<String>,
    last_update: Option<String>,
    name: Option<String>,
    description: Option<String>,
}

impl ResourceCandidate {
    /// Create a new ResourceCandidate instance
    pub fn new(name : impl Into<String>) -> ResourceCandidate {
        let mut rc = ResourceCandidate::create_with_time();
        rc.name = Some(name.into());
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
            id: value.get_id(), 
            href: value.get_href(), 
            name: value.get_name(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::HasName;

    const CANDIDATE_NAME : &str = "ResourceCandidateName";
    const CANDIDATE_DESC : &str = "CandidateDescription";

    #[test]
    fn test_resourcecandidate_new() {
        let candidate = ResourceCandidate::new(CANDIDATE_NAME);

        assert_eq!(candidate.get_name().as_str(),CANDIDATE_NAME);
    }

    #[test]
    fn test_resourcecandidate_desc() {
        let candidate = ResourceCandidate::new(CANDIDATE_NAME)
        .description(CANDIDATE_DESC);

        assert_eq!(candidate.description.is_some(),true);
        assert_eq!(candidate.description.unwrap().as_str(),CANDIDATE_DESC);
    }

    #[test]
    fn test_candidateref_from_candidate() {
        let candidate = ResourceCandidate::new(CANDIDATE_NAME);

        let candidate_ref = ResourceCandidateRef::from(candidate.clone());

        assert_eq!(candidate.get_id(),candidate_ref.id);
        assert_eq!(candidate.get_href(),candidate_ref.href);
        assert_eq!(candidate.get_name(),candidate_ref.name);
    }
}