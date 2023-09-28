//! Note Module

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    id: String,
    author: Option<String>,
    date: Option<String>,
    text: Option<String>,
}
