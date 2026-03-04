use serde::{Deserialize, Serialize};

///Policy Constraint MVO
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyConstraintMvo(pub serde_json::Value);
