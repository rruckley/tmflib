use serde::{Serialize, Deserialize};
///Container for PolicyAction Reference or unmanaged PolicyAction object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyActionRefOrValueFvo {}
impl std::fmt::Display for PolicyActionRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
