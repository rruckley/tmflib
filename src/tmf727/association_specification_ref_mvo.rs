use serde::{Serialize, Deserialize};
///reference to an AssociationSpecification object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssociationSpecificationRefMvo {}
impl std::fmt::Display for AssociationSpecificationRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
