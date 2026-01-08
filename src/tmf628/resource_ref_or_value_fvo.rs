use serde::{Deserialize, Serialize};
///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Resource entity and not the ResourceRefOrValue class itself
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceRefOrValueFvo {}
impl std::fmt::Display for ResourceRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
