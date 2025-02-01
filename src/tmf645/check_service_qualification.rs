//! Check Service Qualification Module

use serde::{Deserialize,Serialize};

use crate::{
    common::related_party::RelatedParty, HasDescription, HasId, HasRelatedParty, Uri, LIB_PATH
};

use tmflib_derive::{
    HasId,
    HasDescription,
    HasRelatedParty,
};

const CLASS_PATH : &str = "checkServiceQualification";
use super::{TaskStateType, MOD_PATH};

/// Check Service Qualification
#[derive(Clone,Debug,Default,HasId,HasDescription,HasRelatedParty, Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckServiceQualificaitonItem {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id : Option<String>,
    /// HTTP URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description : Option<String>,
    // Linked Objects
    /// Related Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party : Option<Vec<RelatedParty>>,
}

/// Check Service Qualification 
#[derive(Clone,Debug,Default,HasId,HasDescription,HasRelatedParty,Deserialize,Serialize)]
pub struct CheckServiceQualification {
    /// Unique Id
    pub id : Option<String>,
    /// HTTP Uri
    pub href : Option<Uri>,
    /// Description
    pub description : Option<String>,
    /// SQ Status
    pub state : Option<TaskStateType>,
        // Referenced modules
    /// Service Qualification Items
    pub service_qualification_item : Option<Vec<CheckServiceQualificaitonItem>>,

    /// Related Parties
    pub related_party : Option<Vec<RelatedParty>>,
}

impl CheckServiceQualification {
    /// Create a new SQ Check
    pub fn new(desc : impl Into<String>) -> CheckServiceQualification {
        CheckServiceQualification {
            ..CheckServiceQualification::create()
        }
        .description(desc)
        .state(TaskStateType::default())
    }

    /// Set the status
    pub fn state(mut self, state : TaskStateType) -> CheckServiceQualification {
        self.state = Some(state);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SQ_DESC : &str = "SQ Description";

    #[test]
    fn test_sq_create() {
        let sq = CheckServiceQualification::new(SQ_DESC);

        assert_eq!(sq.description.is_some(),true);
        assert_eq!(sq.description.unwrap(),SQ_DESC.to_string());
    }
}