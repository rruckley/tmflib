use serde::{Serialize, Deserialize};
///A PolicyVariable is an entity for modeling different types of variables that can be used to form a PolicyCondition statement. It can be static or dynamic.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyVariable {}
impl std::fmt::Display for PolicyVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
