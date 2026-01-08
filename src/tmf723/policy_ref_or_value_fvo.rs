use serde::{Serialize, Deserialize};
///Container for Policy Reference or unmanaged Policy object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyRefOrValueFvo {}
impl std::fmt::Display for PolicyRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
