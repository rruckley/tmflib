//! Note Module

use super::extensible::Extensible;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::convert::From;
use uuid::Uuid;

/// A note is a comment, observation, or remark made by a user or system about a particular entity. It typically contains textual information that provides additional context, insights, or feedback related to the entity it is associated with. Notes can be used for various purposes, such as documenting important details, sharing observations, or recording feedback.
#[derive(Clone, Debug, Deserialize, PartialEq, Default,Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
        ///Base Extensible schema for use in TMForum Open-APIs - When used for in a schema it means that the Entity described by the schema  MUST be extended with the @type
    #[serde(flatten)]
    pub extensible: Extensible,
    ///When sub-classing, this defines the super-class
    #[serde(rename = "@baseType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_type: Option<String>,
    ///A URI to a JSON-Schema file that defines additional attributes and relationships
    #[serde(rename = "@schemaLocation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_location: Option<String>,
    ///When sub-classing, this defines the sub-class Extensible name
    #[serde(rename = "@type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    ///Author of the note
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    ///Date of the note
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::DateTime>,
    ///Identifier of the note within its containing entity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    ///Text of the note
    pub text: Option<String>,
}
impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Note {
    /// Create a new note, without author
    pub fn new(text: impl Into<String>) -> Note {
        let id = Uuid::new_v4().simple().to_string();
        let now = Utc::now();
        let time = chrono::DateTime::from_timestamp(now.timestamp(), 0).unwrap();
        Note {
            id: Some(id),
            author: None,
            date: Some(time.to_string()),
            text: Some(text.into()),
            ..Default::default()
        }
    }
    /// Set author for note with builder pattern
    pub fn author(mut self, author: &str) -> Note {
        self.author = Some(author.to_string());
        self
    }
}

impl From<&str> for Note {
    fn from(value: &str) -> Self {
        Note::new(value)
    }
}

impl std::ops::Deref for Note {
    type Target = Extensible;
    fn deref(&self) -> &Self::Target {
        &self.extensible
    }
}
impl std::ops::DerefMut for Note {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.extensible
    }
}

#[cfg(test)]
mod test {
    use super::Note;

    #[test]
    fn test_note_create_str() {
        let note = Note::from("StringSlice");

        assert_eq!(note.text.is_some(), true);
        assert_eq!(note.text.unwrap(), "StringSlice");
    }
    #[test]
    fn test_note_create_author() {
        let note = Note::from("StringSlice").author("AnAuthor");

        assert_eq!(note.author.is_some(), true);
        assert_eq!(note.author.unwrap(), "AnAuthor".to_string());
    }  
}
