use serde::{Serialize, Deserialize};
///An item typically included in a request or response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HeaderItem {
    ///The name of the header item, e.g. locale
    pub name: String,
    ///The value of the header item, e.g. en-us
    pub value: String,
}
impl std::fmt::Display for HeaderItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
