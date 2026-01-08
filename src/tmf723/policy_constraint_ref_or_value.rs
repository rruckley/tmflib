use serde::{Serialize, Deserialize};
///Container for PolicyConstraint Reference or unmanaged PolicyConstraint object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyConstraintRefOrValue {}
impl std::fmt::Display for PolicyConstraintRefOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
