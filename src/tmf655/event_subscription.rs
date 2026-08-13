use serde::{Serialize, Deserialize};
///Sets the communication endpoint address the service instance must use to deliver notification information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventSubscription {
    ///The callback being registered.
    pub callback: String,
    ///Id of the listener
    pub id: String,
    ///additional data to be passed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}
impl std::fmt::Display for EventSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
