//! Note Module

use serde::{Deserialize, Serialize};
use chrono::naive::NaiveDateTime;
use chrono::Utc;
use uuid::Uuid;
use std::convert::From;
use crate::DateTime;

/// Notes object for journaling against many TMF objects
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    id: String,
    author: Option<String>,
    date: Option<DateTime>,
    text: Option<String>,
}

impl Note {
    /// Create a new note, without author
    pub fn new(text : impl Into<String>) -> Note {
        let id = Uuid::new_v4().simple().to_string();
        let now = Utc::now();
        let time = NaiveDateTime::from_timestamp_opt(now.timestamp(), 0).unwrap();
        Note { 
            id, 
            author: None, 
            date: Some(time.to_string()), 
            text : Some(text.into()),
        }
    }
    /// Set author for note with builder pattern
    pub fn author(mut self, author : &str) -> Note {
        self.author = Some(author.to_string());
        self
    }
}

impl From<&str> for Note {
    fn from(value: &str) -> Self {
        Note::new(value.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::Note;
    #[test]
    fn test_note_create_str() {
        let note = Note::from("StringSlice");

        assert_eq!(note.text.is_some(),true);
        assert_eq!(note.text.unwrap(),"StringSlice");
    }
    #[test]
    fn test_note_create_author() {
        let note = Note::from("StringSlice")
        .author("AnAuthor");

        assert_eq!(note.author.is_some(),true);
        assert_eq!(note.author.unwrap(),"AnAuthor".to_string());
    }
}
