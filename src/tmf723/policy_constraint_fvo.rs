use serde::{Serialize, Deserialize};

///Policy Constraint MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyConstraintFvo(pub serde_json::Value);
