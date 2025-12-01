use serde::{Serialize, Deserialize};

///Reference to an alarm for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlarmRef {}
impl std::fmt::Display for AlarmRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
