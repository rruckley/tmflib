use serde::{Serialize, Deserialize};
///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Resource entity and not the ResourceRefOrValue class itself
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceRefOrValueMvo {}
impl std::fmt::Display for ResourceRefOrValueMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
