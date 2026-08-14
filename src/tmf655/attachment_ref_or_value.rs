use super::Quantity;
use crate::TimePeriod;
use serde::{Deserialize, Serialize};
///An attachment by value or by reference. An attachment complements the description of an element, for example through a document, a video, a picture.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttachmentRefOrValue {
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///The actual type of the target instance when needed for disambiguation.
    #[serde(rename = "@referredType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referred_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///Attachment type such as video, picture
    #[serde(rename = "attachmentType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_type: Option<String>,
    ///The actual contents of the attachment object, if embedded, encoded as base64
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    ///A narrative text describing the content of the attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///URI for this Attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    ///Unique identifier for this particular attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Attachment mime type such as extension file for video, picture and document
    #[serde(rename = "mimeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    ///The name of the attachment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///An amount in a given unit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<Quantity>,
    ///Uniform Resource Locator, is a web page address (a subset of URI)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
}
impl std::fmt::Display for AttachmentRefOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
