
//! Intent Module

use crate::{TimePeriod,HasId,HasName,HasDescription,LIB_PATH};
use serde::{Deserialize, Serialize};
use tmflib_derive::{HasName,HasId,HasDescription};
use super::MOD_PATH;

const CLASS_PATH : &str = "intent";

/// Represents an Intent with various attributes such as id, href, name, description, version, status, and valid_for period.
/// 
/// # Fields
/// 
/// * `id` - An optional unique identifier for the intent.
/// * `href` - An optional hyperlink reference to the intent.
/// * `name` - An optional name for the intent.
/// * `description` - An optional description of the intent.
/// * `version` - The version of the intent.
/// * `status` - The current status of the intent.
/// * `valid_for` - An optional time period during which the intent is valid.
#[derive(Clone, Debug, Default, Deserialize, Serialize, HasId,HasName, HasDescription)]
pub struct Intent {
    /// An optional unique identifier for the intent.
    pub id: Option<String>,
    /// An optional hyperlink reference to the intent.
    pub href: Option<String>,
    /// An optional name for the intent.
    pub name: Option<String>,
    /// An optional description of the intent.
    pub description: Option<String>,
    /// The version of the intent.
    pub version: String,
    /// The current status of the intent.
    pub status: String,
    /// An optional time period during which the intent is valid.
    pub valid_for: Option<TimePeriod>,
}

