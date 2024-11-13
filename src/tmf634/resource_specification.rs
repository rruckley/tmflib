//! Resource Specification Module

use serde::{Deserialize,Serialize};
use crate::{
    DateTime, 
    HasDescription, 
    HasId, 
    HasLastUpdate, 
    HasName, 
    HasValidity, 
    TimePeriod,  
    Uri,
    LIB_PATH
};

use tmflib_derive::{
    HasId,
    HasName,
    HasDescription,
    HasLastUpdate,
    HasValidity
};

use crate::common::attachment::AttachmentRefOrValue;
use crate::common::external_identifier::ExternalIdentifier;

use super::MOD_PATH;

const CLASS_PATH: &str = "resourceSpecification";

/// Resource Specification
#[derive(Clone,Default,Debug,Deserialize,HasId,HasName,HasDescription,HasLastUpdate,HasValidity,Serialize)]
pub struct ResourceSpecification {
    /// Resource Specification Category
    pub category: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Uri for this specification
    pub href: Option<Uri>,
    /// Unique Id of this specification
    pub id: Option<String>,
    /// Is this part of a bundle?
    pub is_bundle : bool,
    /// When was this recort last updated
    pub last_update: Option<DateTime>,
    /// Current lifecycle status
    pub lifecycle_status : Option<String>,
    /// Name of the specification
    pub name : Option<String>,
    /// Validity for this specification
    pub valid_for: Option<TimePeriod>,
    // Referenced types
    /// Attachments
    pub attachment : Vec<AttachmentRefOrValue>,
    /// External identifiers
    pub external_identifier : Vec<ExternalIdentifier>,
    /// Features
    pub feature : Vec<FeatureSpecification>,
}

impl ResourceSpecification {
    /// Create a new specification with the given name
    pub fn new(name : impl Into<String>) -> ResourceSpecification {
        ResourceSpecification {
            name : Some(name.into()),
            lifecycle_status : Some(String::from("New")),
            ..ResourceSpecification::create_with_time()
        }
    }
}

/// Feature Specification for Resource
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct FeatureSpecification {
    /// Specificaiton Relationships
    pub feature_spec_relationship : Vec<FeatureSpecificationRelationship>
}

/// Specification Relationship Type
#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct FeatureSpecificationRelationship {}

#[cfg(test)]
mod test {
    use super::*;

    const SPEC_NAME : &str = "SpecificationName";

    #[test]
    fn test_resourcespecification_new() {
        let spec = ResourceSpecification::new(SPEC_NAME);

        assert_eq!(spec.name.is_some(),true);
        assert_eq!(spec.get_name().as_str(),SPEC_NAME);
    }
}