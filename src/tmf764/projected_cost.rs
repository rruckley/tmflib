//! Projected Cost Module

use serde::{Deserialize, Serialize};

use crate::{HasId, Uri};

use super::MOD_PATH;
use tmflib_derive::HasId;

const CLASS_PATH: &str = "ProjectedCost";

/// Projected Cost Item
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProjectedCostItem {}

/// Projected Cost
#[derive(Debug, Default, Clone, HasId, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedCost {
    /// Unique identifier
    pub id: Option<String>,
    /// Hyperlink reference
    pub href: Option<Uri>,
    /// Cost Items
    pub cost_item: Option<Vec<ProjectedCostItem>>,
}
