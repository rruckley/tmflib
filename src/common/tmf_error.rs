//! Error Module for TMF

use thiserror::Error;
use regex::Error as RegexError;

/// TMF Error Enum
/// This enum defines various error types that can occur in the TMF library.
#[derive(Error, Debug)]
pub enum TMFError {
    /// Generic error with a message
    #[error("Error : {0}")]
    GenericError(String),
    /// Invalid ID error
    #[error("Invalid ID: {0}")]
    InvalidId(String),
    /// Invalid Name error
    #[error("Invalid Name: {0}")]
    InvalidName(String),
    /// Invalid Reference error
    #[error("Invalid Reference: {0}")]
    InvalidReference(String),
    /// Invalid Time Period error
    #[error("Invalid Time Period: {0}")]
    InvalidTimePeriod(String),
    /// Invalid Date error
    #[error("Event Error: {0}")]
    EventError(String),
    /// Invalid Event Payload error
    #[error("Characteristic Error: {0}")]
    CharacteristicError(String),
    /// Invalid Related Party error
    #[error("Related Party Error: {0}")]
    RelatedPartyError(String),
    /// Contact Medium Error
    #[error("Contact Medium Error: {0}")]
    ContactMediumError(String),
    /// Currency Error
    #[error("Bad Currency: {0}")]
    CurrencyError(String),
    /// Invalid Note error
    #[error("No data present for {0}")]
    NoDataError(String),
    /// Regex Error
    #[error("Regex Error: {0}")]
    RegexError(#[from] RegexError),
}

impl From<&str> for TMFError {
    fn from(msg: &str) -> Self {
        TMFError::GenericError(msg.to_string())
    }
}
