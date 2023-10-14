//! Note Module

use serde::{Deserialize, Serialize};

/// Notes object for journaling against many TMF objects
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    id: String,
    author: Option<String>,
    date: Option<String>,
    text: Option<String>,
}
