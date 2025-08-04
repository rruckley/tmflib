//! Tax Item

use super::money::Money;
use serde::{Deserialize, Serialize};

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

    const TAX_JSON: &str = "{
        \"taxCategory\" : \"TaxCategory\",
        \"taxRate\" : 0.10,
        \"taxAmount\" : { \"unit\" : \"AUD\", \"value\": 100.0}
    }";

    #[test]
    fn test_taxitem_deserialise() {
        let taxitem: TaxItem = serde_json::from_str(TAX_JSON).expect("Could not parase TAX_JSON");

        assert_eq!(taxitem.tax_category.as_str(), "TaxCategory");
        assert_eq!(taxitem.tax_rate, 0.10);
    }
}
