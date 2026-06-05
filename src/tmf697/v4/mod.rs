//! V4 Modules
//!

const TMF_MODULE: &str = "workOrderManagement";

pub mod work;
pub mod work_order;
pub mod work_order_item;

/// Modules available for this API `MOD_PATH`
pub fn get_objects() -> Vec<&'static str> {
    vec![work::CLASS_PATH, work_order::CLASS_PATH]
}
