use serde::{Serialize, Deserialize};
///Container for PolicyVariable Reference or unmanaged PolicyVariable object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyVariableRefOrValueMvo {}
impl std::fmt::Display for PolicyVariableRefOrValueMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
