use serde::{Serialize, Deserialize};

///A placeholder for any type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Any {}
impl std::fmt::Display for Any {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
