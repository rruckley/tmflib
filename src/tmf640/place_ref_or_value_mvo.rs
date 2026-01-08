use serde::{Deserialize, Serialize};
///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Place entity and not the PlaceRefOrValue class itself
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaceRefOrValueMvo {}
impl std::fmt::Display for PlaceRefOrValueMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
