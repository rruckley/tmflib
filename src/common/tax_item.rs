//! Tax Item

use serde::{Deserialize,Serialize};
use super::money::Money;

/// Tax Details
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxItem {
    tax_category: String,
    tax_rate: f32,
    tax_amount: Money,
}