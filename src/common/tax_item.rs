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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_taxitem_deserialise() {
        let tax = TaxItem::default();
        let tax_str = serde_json::to_string(&tax);

        assert_eq!(tax_str.is_ok(),true);
    }
}