use serde::{Deserialize, Serialize};
/// Document Reference or Document Value MVO     
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentRefOrValueMvo {}
impl std::fmt::Display for DocumentRefOrValueMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
