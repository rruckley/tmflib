use serde::{Deserialize, Serialize};
///The polymorphic attributes @type, @schemaLocation & @referredType are related to the Attachment entity and not the AttachmentRefOrValue class itself
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachmentRefOrValueFvo {}
impl std::fmt::Display for AttachmentRefOrValueFvo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
