use serde::{Serialize, Deserialize};
///reference to an AssociationSpecification object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssociationSpecificationRef {}
impl std::fmt::Display for AssociationSpecificationRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
