//! Service Candidate Modules

use serde::{Deserialize, Serialize};
use std::convert::From;

use crate::{vec_insert, HasId, HasLastUpdate, HasName, TimePeriod, TimeStamp, Uri, LIB_PATH};
use tmflib_derive::{HasId, HasLastUpdate, HasName};

use super::{
    service_category::ServiceCategoryRef, service_specification::ServiceSpecificationRef, MOD_PATH,
};
const CLASS_PATH: &str = "serviceCandidate";
const CANDIDATE_NEW_VERS: &str = "1.0";
const CANDIDATE_NEW_STATUS: &str = "new";

/// Service Candidate
#[derive(Clone, Debug, Default, Deserialize, HasId, HasLastUpdate, HasName, Serialize)]
pub struct ServiceCandidate {
    id: Option<String>,
    href: Option<Uri>,
    name: Option<String>,
    last_update: Option<TimeStamp>,
    lifecycle_status: Option<String>,
    valid_for: Option<TimePeriod>,
    version: Option<String>,
    // References
    service_specification: ServiceSpecificationRef,
    category: Option<Vec<ServiceCategoryRef>>,
}

impl ServiceCandidate {
    /// Create new instance of Service Candidate
    pub fn new(
        name: impl Into<String>,
        specification_ref: ServiceSpecificationRef,
    ) -> ServiceCandidate {
        ServiceCandidate {
            name: Some(name.into()),
            lifecycle_status: Some(CANDIDATE_NEW_STATUS.into()),
            version: Some(CANDIDATE_NEW_VERS.into()),
            service_specification: specification_ref,
            ..ServiceCandidate::create_with_time()
        }
    }

    /// Add a category to this Service Candidate by passing in a Category reference
    pub fn category(mut self, category: ServiceCategoryRef) -> ServiceCandidate {
        vec_insert(&mut self.category, category);
        self
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

    const CANDIDATE_NAME: &str = "CandidateName";
    #[test]
    fn test_servicecandidate_new() {
        // Since new() requires a specification, using deafult then setting name via set_name()
        let mut candidate = ServiceCandidate::default();
        candidate.generate_id();
        candidate.set_name(CANDIDATE_NAME);

        assert_eq!(candidate.get_name().as_str(), CANDIDATE_NAME);
        assert_eq!(candidate.id.is_some(), true);
        assert_eq!(candidate.href.is_some(), true);
    }

    #[test]
    fn test_servicecandidate_from_string() {
        // Since new() requires a specification, using deafult then setting name via set_name()
        let mut candidate = ServiceCandidate::default();
        candidate.generate_id();
        candidate.set_name(CANDIDATE_NAME);

        assert_eq!(candidate.get_name().as_str(), CANDIDATE_NAME);
        assert_eq!(candidate.id.is_some(), true);
        assert_eq!(candidate.href.is_some(), true);
    }

    #[test]
    fn test_candidateref_from_candidate() {
        // Since new() requires a specification, using deafult then setting name via set_name()
        let mut candidate = ServiceCandidate::default();
        candidate.generate_id();
        candidate.set_name(CANDIDATE_NAME);

        let candidate_ref = ServiceCandidateRef::from(candidate.clone());

        assert_eq!(candidate.get_name(), candidate_ref.name);
        assert_eq!(candidate.get_id(), candidate_ref.id);
        assert_eq!(candidate.get_href(), candidate_ref.href);
    }
}
