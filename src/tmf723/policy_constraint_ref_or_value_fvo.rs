use serde::{Serialize, Deserialize};
///Container for PolicyConstraint Reference or unmanaged PolicyConstraint object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyConstraintRefOrValueFvo {}
impl std::fmt::Display for PolicyConstraintRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
