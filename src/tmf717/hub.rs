use serde::{Serialize, Deserialize};
///Sets the communication endpoint address the service instance must use to deliver notification information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Hub {}
impl std::fmt::Display for Hub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
