//! Actual Cost

use serde::{Deserialize, Serialize};

use crate::{
    HasId,
    Uri,
};

use tmflib_derive::HasId;

const CLASS_PATH : &str = "ActualCost";
use super::MOD_PATH;

/// Actual Cost Item
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ActualCostItem {}

/// Actual Cost
#[derive(Debug, Default, Clone,HasId, Serialize, Deserialize)]
pub struct ActualCost {
    /// Unique identifier
    pub id: Option<String>,
    /// Hyperlink reference
    pub href : Option<Uri>,
    /// Cost Items
    pub cost_item: Option<Vec<ActualCostItem>>,
}