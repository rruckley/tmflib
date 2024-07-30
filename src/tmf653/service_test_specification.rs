//! Service Test Specification Module

use crate::{HasId,HasName, Uri,LIB_PATH};
use tmflib_derive::{HasId,HasName};

use serde::{Deserialize,Serialize};

use super::MOD_PATH;
const CLASS_PATH: &str = "specification";

/// Service Test Specification
#[derive(Clone,Debug,Default,Deserialize,HasId,HasName,Serialize)]
pub struct ServiceTestSpecification {
    /// Unique Identifier
    pub id: Option<String>,
    /// HREF to specification
    pub href: Option<Uri>,
    /// Name
    pub name: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    const SPECIFICATION_ID : &str = "STS123";
    const SPECIFICATION_NAME : &str = "SpecificationName";

    const SPECIFICATION_JSON : &str = "{
        \"id\" : \"STS123\",
        \"name\" : \"SpecificationName\"
    }";

    #[test]
    fn test_specification_deserialize() {
        let specification : ServiceTestSpecification = serde_json::from_str(SPECIFICATION_JSON).unwrap();

        assert_eq!(specification.get_id().as_str(),"STS123");
        assert_eq!(specification.get_name().as_str(),"SpecificationName");
    }

    #[test]
    fn test_specification_hasid() {
        let mut specification = ServiceTestSpecification::default();
        specification.set_id(SPECIFICATION_ID);

        assert_eq!(specification.get_id().as_str(),SPECIFICATION_ID);
    }

    #[test]
    fn test_specification_hasname() {
        let mut specification = ServiceTestSpecification::default();

        specification.set_name(SPECIFICATION_NAME);

        assert_eq!(specification.get_name().as_str(),SPECIFICATION_NAME);
    }
}