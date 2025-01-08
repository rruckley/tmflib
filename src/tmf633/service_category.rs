//! Service Category Module


use serde::{Deserialize, Serialize};

use crate::{
    HasId, 
    HasLastUpdate, 
    HasName, 
    HasDescription,
    HasValidity,
    TimeStamp, 
    TimePeriod,
    LIB_PATH,
    Uri,
};
use tmflib_derive::{HasId, HasLastUpdate, HasDescription, HasName, HasValidity};

use super::{service_candidate::ServiceCandidate, MOD_PATH};
const CLASS_PATH : &str = "serviceCategory";
const CAT_STATUS_NEW : &str = "new";
const CAT_VERS_NEW : &str = "1.0";

/// Service Category Reference
/// # Description
/// Reference to another service category in the catalog
#[derive(Clone,Default,Debug,Deserialize, Serialize)]
pub struct ServiceCategoryRef {
    href: Uri,
    id: String,
    name: String,
    version: Option<String>,
}

impl From<ServiceCategory> for ServiceCategoryRef {
    fn from(value: ServiceCategory) -> Self {
        ServiceCategoryRef {
            href: value.get_href(),
            id: value.get_id(),
            name: value.get_name(),
            version: value.version.clone(),
        }
    }
}

/// Service Category
/// # Desecription
/// Categorisation for services in a service catalog
#[derive(Clone, Debug, Default, Deserialize, HasId, HasName, HasDescription, HasLastUpdate, HasValidity, Serialize)]
pub struct ServiceCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<Uri>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_root: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_update: Option<TimeStamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    valid_for: Option<TimePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
        // META
    /// Base Type this type is derived from if creating sub-classes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    pub base_type : Option<String>,
    /// Schema Definition of the sub-class (if required)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    pub schema_location: Option<Uri>,
    /// Name for this Type when sub-classing
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type : Option<String>,
    // References
    #[serde(skip_serializing_if = "Option::is_none")]
    category : Option<Vec<ServiceCategoryRef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_candidate: Option<Vec<ServiceCandidate>>,
}

impl ServiceCategory {
    /// Create a new category instance
    pub fn new(name : impl Into<String>) -> ServiceCategory {
        ServiceCategory {
            name: Some(name.into()),
            lifecycle_status: Some(CAT_STATUS_NEW.into()),
            version : Some(CAT_VERS_NEW.into()),
            ..ServiceCategory::create_with_time()
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const CAT_NAME : &str = "CAT_NAME";

    #[test]
    fn test_servicecategory_create() {
        let category = ServiceCategory::new(CAT_NAME);

        assert_eq!(category.get_name(),CAT_NAME.to_string());
    }

    #[test]
    fn test_category_into_ref() {
        let category = ServiceCategory::new(CAT_NAME);

        let cat_ref : ServiceCategoryRef = category.into();

        // name in ref should match input name of cat
        assert_eq!(CAT_NAME.to_string(),cat_ref.name);
    }
}