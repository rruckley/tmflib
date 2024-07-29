//! Work Order Item Module
//! 
//! This module defines an item as part of an overall [`super::work_order::WorkOrder`]

use serde::{Deserialize,Serialize};
use crate::gen_code;

use super::work::WorkRefOrValue;

/// Work Order Item
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkOrderItem {
    /// Metadata: Type of schema, same as [WorkOrderItem::base_type] if aligned to TMF specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type: Option<String>,
    /// Metadata: Parent type of schema for derived types
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@baseType")]
    pub base_type: Option<String>,
    /// Metadata: Location of schema if derived schema used, i.e. @type differs from @base_type
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@schemaLocation")]
    pub schema_location: Option<String>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Reference of Value for Work
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work: Option<WorkRefOrValue>,
}

impl WorkOrderItem {
    /// Create new WorkOrderItem based on some Work
    /// ```
    /// use tmflib::tmf697::v4::work::{WorkRefOrValue,Work};
    /// use tmflib::tmf697::v4::work_order_item::WorkOrderItem;
    /// let work = Work::new("Some Work");
    /// let woi = WorkOrderItem::with(WorkRefOrValue::from(work));
    /// ```
    pub fn with(work : WorkRefOrValue) -> WorkOrderItem {
        WorkOrderItem {
            id : Some(gen_code(work.get_name(), work.get_id(), None, Some("WI-".to_string()), Some(7)).0),
            work : Some(work),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const WORK_ITEM_JSON: &str = "{
        \"id\" : \"WI_123\"
    }";



    #[test]
    fn test_workorderitem_deserialize() {
        let workorderitem : WorkOrderItem = serde_json::from_str(WORK_ITEM_JSON).unwrap();

        assert_eq!(workorderitem.id.is_some(),true);
        assert_eq!(workorderitem.id.unwrap().as_str(),"WI_123");
    }

}

