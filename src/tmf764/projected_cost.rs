//! Projected Cost Module

use serde::{Deserialize, Serialize};

use crate::{
    common::{attachment::AttachmentRefOrValue, note::Note, related_party::RelatedParty},
    vec_insert, HasAttachment, HasDescription, HasId, HasLastUpdate, HasName, HasNote,
    HasRelatedParty, IsAddressable, TMFError, Uri,
};

use super::MOD_PATH;
use tmflib_derive::{
    HasAttachment, HasDescription, HasId, HasLastUpdate, HasName, HasNote, HasRelatedParty,
};

/// Path to module
pub const CLASS_PATH: &str = "ProjectedCost";

/// Projected Cost Item
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProjectedCostItem {
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Projected Cost
#[derive(
    Debug,
    Default,
    Clone,
    HasId,
    HasName,
    HasAttachment,
    HasDescription,
    HasLastUpdate,
    HasNote,
    HasRelatedParty,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedCost {
    /// Unique identifier
    pub id: Option<String>,
    /// Hyperlink reference
    pub href: Option<Uri>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Cost Items
    pub cost_item: Option<Vec<ProjectedCostItem>>,

    /// Notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<Note>>,

    /// Related Parties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_party: Option<Vec<RelatedParty>>,

    /// Last Update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<crate::DateTime>,
    /// Attachments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Vec<AttachmentRefOrValue>>,
}

impl ProjectedCost {
    /// Create a new Actual Cost
    pub fn new(name: impl Into<String>) -> Self {
        Self::create_with_time().name(name.into())
    }

    /// Add an Actual Cost Item
    ///
    /// # Arguments
    /// * `item` - The Actual Cost Item to add
    /// # Returns
    /// * `Self` - The updated Actual Cost
    /// # Example
    /// ```rust
    /// use tmflib::tmf764::actual_cost::{ActualCost, ActualCostItem};
    /// let cost = ActualCost::new("Example Cost")
    ///     .item(ActualCostItem::new("Item 1"));
    /// ```
    #[must_use]
    pub fn item(mut self, item: ProjectedCostItem) -> Self {
        vec_insert(&mut self.cost_item, item);
        self
    }
}

impl IsAddressable for ProjectedCost {
    fn get_objects() -> Vec<&'static str> {
        super::get_objects()
    }
}

impl From<ProjectedCost> for super::CostRef {
    fn from(cost: ProjectedCost) -> Self {
        Self {
            id: cost.get_id(),
            href: cost.get_href(),
            name: cost.name,
            base_type: None,
            referred_type: None,
            schema_location: None,
            r#type: Some(String::from("ProjectedCost")),
        }
    }
}
