//! Work Order Item Module
//! 
//! Covers the container object for inclusion in a [`WorkOrder`]

use serde::{Deserialize,Serialize};
use super::work::WorkRefOrValue;
use crate::gen_code;

/// Work Order Item
#[derive(Clone,Debug,Default,Deserialize,Serialize)]
pub struct WorkOrderItem {
    /// Metadata: Type of schema, same as [`base_type`] if aligned to TMF specification.
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
    /// #[cfg(feature = "tmf697-v4")]
    /// use tmflib::tmf697::v4::work::{WorkRefOrValue,Work};
    /// #[cfg(feature = "tmf697-v5")]
    /// use tmflib::tmf697::v5::work::{WorkRefOrValue,Work};
    /// #[cfg(feature = "tmf697-v4")]
    /// use tmflib::tmf697::v4::work_order_item::WorkOrderItem;
    /// #[cfg(feature = "tmf697-v5")]
    /// use tmflib::tmf697::v5::work_order_item::WorkOrderItem;
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