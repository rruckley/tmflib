use serde::{Serialize, Deserialize};

/// PolicyConstraint represents a managed policy constraint.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyConstraint(pub serde_json::Value);
