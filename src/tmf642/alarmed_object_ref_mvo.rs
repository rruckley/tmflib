use serde::{Serialize, Deserialize};
///Reference to object which affected by Alarm (AlarmedObject).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlarmedObjectRefMvo {}
impl std::fmt::Display for AlarmedObjectRefMvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
