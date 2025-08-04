//! Product Offering Qualificaiton Item Module
//!
//!
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::note::Note;
#[cfg(all(feature = "tmf620", feature = "build-V4"))]
use crate::tmf620::product_offering::ProductOfferingRef;
#[cfg(all(feature = "tmf620", feature = "build-V5"))]
use crate::tmf620::product_offering_v5::ProductOfferingRef;

use super::product_qualification::TaskStateType;

/// Action for this product offering
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub enum ProductActionType {
    /// Add a new offering
    Add,
    /// Modify an exiting offering
    Modify,
    /// Delete an offering
    Delete,
    /// Make no change
    #[default]
    NoChange,
}

/// Product Offering Qualification
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductOfferingQualificationItem {
    /// Action for this qualification
    pub action: ProductActionType,
    /// Unique Id for this qualification
    pub id: Option<String>,
    /// Notes for this qualification
    pub note: Vec<Note>,
    /// Status of this qualification
    pub state: TaskStateType,
    /// Product Offerings in scope for this qualification item
    pub product_offering: Option<ProductOfferingRef>,
}

impl ProductOfferingQualificationItem {
    /// Create a new Product Offering Qualification Item
    pub fn new() -> ProductOfferingQualificationItem {
        let id = Uuid::new_v4().simple().to_string();
        ProductOfferingQualificationItem {
            id: Some(id),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ProductActionType, ProductOfferingQualificationItem};
    use crate::tmf679::product_qualification::TaskStateType;

    const ACTIONTYPE: &str = "\"NoChange\"";

    const POQI_JSON: &str = "{
        \"action\" : \"NoChange\",
        \"note\" : [],
        \"state\" : \"InProgress\"
    }";

    #[test]
    fn test_actiontype_deserialize() {
        let actiontype: ProductActionType = serde_json::from_str(ACTIONTYPE).unwrap();

        assert_eq!(actiontype, ProductActionType::NoChange);
    }

    #[test]
    fn test_poqi_deserialize() {
        let poqi: ProductOfferingQualificationItem = serde_json::from_str(POQI_JSON).unwrap();

        assert_eq!(poqi.action, ProductActionType::NoChange);
        assert_eq!(poqi.state, TaskStateType::InProgress);
    }

    #[test]
    fn test_poqi_new() {
        let poqi = ProductOfferingQualificationItem::new();

        assert_eq!(poqi.id.is_some(), true);
    }
}
