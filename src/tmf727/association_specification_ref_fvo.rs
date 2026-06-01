use serde::{Deserialize, Serialize};
///reference to an `AssociationSpecification` object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssociationSpecificationRefFvo {}
impl std::fmt::Display for AssociationSpecificationRefFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
