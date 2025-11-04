//! Actual Cost

use serde::{Deserialize, Serialize};

use crate::{
    HasAttachment, HasDescription, HasId, HasLastUpdate, HasName, HasNote, HasRelatedParty, TMFError, Uri, common::{attachment::AttachmentRefOrValue, note::Note, related_party::RelatedParty},vec_insert
};

use tmflib_derive::{HasAttachment, HasDescription, HasId, HasLastUpdate, HasName, HasNote, HasRelatedParty};

const CLASS_PATH: &str = "ActualCost";
use super::MOD_PATH;

/// Actual Cost Item
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ActualCostItem {}

/// Cost Relationship
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CostRelationship {}

/// Actual Cost
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
pub struct ActualCost {
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Hyperlink reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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
    /// Cost Items
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_item: Option<Vec<ActualCostItem>>,
    /// Cost Relationships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_relationship: Option<Vec<CostRelationship>>,
}

impl ActualCost {
    /// Create a new Actual Cost
    pub fn new(name : impl Into<String>) -> Self {
        Self::create_with_time()
            .name(name.into())
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
    ///     .item(ActualCostItem{});
    /// ``` 
    pub fn item(mut self, item: ActualCostItem) -> Self {
        vec_insert(&mut self.cost_item, item);
        self
    }   
}
