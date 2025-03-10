
//! Intent Module

use crate::{DateTime, HasDescription, HasId, HasName, TimePeriod, LIB_PATH};
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
#[serde(rename_all = "camelCase")]
pub struct Intent {
    /// An optional unique identifier for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// An optional hyperlink reference to the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// An optional name for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// An optional description of the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The current status of the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The date and time when the status was last changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_change_date: Option<DateTime>,
    /// An optional time period during which the intent is valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    /// The version of the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

