use serde::{Serialize, Deserialize};
///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Attachment entity and not the AttachmentRefOrValue class itself
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachmentRefOrValue {}
impl std::fmt::Display for AttachmentRefOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
