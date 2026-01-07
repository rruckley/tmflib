use serde::{Serialize, Deserialize};
///Reference of a CalendarEvent
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CalendarEventRef {}
impl std::fmt::Display for CalendarEventRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
