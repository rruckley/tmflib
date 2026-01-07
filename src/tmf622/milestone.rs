//! Product Order Milestone Module

use serde::{Deserialize, Serialize};

/// Product Order Milestones
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Milestone {
    description: String,
    id: String,
    message: String,
    milestone_date: String,
    name: String,
    status: String,
}
