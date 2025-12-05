use serde::{Serialize, Deserialize};
///Intent Ref (if Intent already exists) or Value (if Intent be created or its details be presented)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntentRefOrValueFvo {}
impl std::fmt::Display for IntentRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
