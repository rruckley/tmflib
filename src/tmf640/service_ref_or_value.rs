use serde::{Deserialize, Serialize};
///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Service entity and not the ServiceRefOrValue class itself
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceRefOrValue {}
impl std::fmt::Display for ServiceRefOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
