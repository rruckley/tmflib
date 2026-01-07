use serde::{Serialize, Deserialize};
use super::{Entity, QuoteStateType, TimePeriod};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer360QuoteVo {
    ///Base entity schema for use in TMForum Open-APIs. Property.
    #[serde(flatten)]
    pub entity: Entity,
    ///Used to categorize the quote from a business perspective that can be useful for the CRM system (e.g. "enterprise", "residential", ...)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    ///Description of the quote
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Date when the quote has been completed
    #[serde(rename = "effectiveQuoteCompletionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_quote_completion_date: Option<crate::DateTime>,
    ///this is the date wished by the requester to have the requested quote item delivered
    #[serde(rename = "expectedFulfillmentStartDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_fulfillment_start_date: Option<crate::DateTime>,
    ///This is expected date - from quote supplier - to be able to send back  a response for this quote
    #[serde(rename = "expectedQuoteCompletionDate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_quote_completion_date: Option<crate::DateTime>,
    ///Possible values for the state of the quote
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<QuoteStateType>,
    ///A period of time, either as a deadline (endDateTime only) a startDateTime only, or both
    #[serde(rename = "validFor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for: Option<TimePeriod>,
    ///Quote version - if the Partner rejected the quote but  negotiations still open a new version of the quote is managed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl std::fmt::Display for Customer360QuoteVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for Customer360QuoteVo {
    type Target = Entity;
    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
impl std::ops::DerefMut for Customer360QuoteVo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entity
    }
}
