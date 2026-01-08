//! Agreement Item Module
use crate::{tmf648::quote_item::QuoteItem, HasValidity, TimePeriod};
use serde::{Deserialize, Serialize};

#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use crate::tmf620::product_offering::ProductOfferingRef;

#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use crate::tmf620::product_offering_v5::ProductOfferingRef;
use tmflib_derive::HasValidity;

/// Agreement Item
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementItem {
    term_or_condition: Option<Vec<AgreementTermOrCondition>>,
    product_offering: Option<Vec<ProductOfferingRef>>,
}

impl From<&QuoteItem> for AgreementItem {
    fn from(_value: &QuoteItem) -> Self {
        AgreementItem::default()
    }
}

/// Agreement Item terms and conditions
#[derive(Clone, Default, Debug, Deserialize, HasValidity, Serialize)]
pub struct AgreementTermOrCondition {
    description: String,
    id: String,
    valid_for: Option<TimePeriod>,
}

/// Agreement Item Reference
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgreementItemRef {
    id: String,
    href: String,
    agreement_item_id: String,
    name: String,
}

#[cfg(test)]
mod test {
    use super::{AgreementItem, AgreementItemRef, AgreementTermOrCondition};
    use crate::{tmf648::quote_item::QuoteItem, HasValidity, TimePeriod};

    const AGREEMENTITEM_JSON: &str = "{
        \"termOrCondition\" : [],
        \"productOffering\" : []
    }";

    const AGREEMENTITEMREF_JSON: &str = "{
        \"id\" : \"AI123\",
        \"href\" : \"http://example.com/tmf651/item/AI123\",
        \"agreementItemId\" : \"AI456\",
        \"name\" : \"AgreementItem\"
    }";

    const AGREEMENTTERMORCOND_JSON: &str = "{
        \"description\" : \"Description\",
        \"id\" : \"ATOC123\"
    }";

    #[test]
    fn test_agreementitem_deserialize() {
        let agreementitem: AgreementItem = serde_json::from_str(AGREEMENTITEM_JSON).unwrap();

        assert_eq!(agreementitem.term_or_condition.is_some(), true);
        assert_eq!(agreementitem.product_offering.is_some(), true);
    }

    #[test]
    fn test_agreementitem_from_quoteitem() {
        let quoteitem = QuoteItem::new();

        let _agreement_item = AgreementItem::from(&quoteitem);
    }

    #[test]
    fn test_agreementtermorcond_deserialize() {
        let termorcond: AgreementTermOrCondition =
            serde_json::from_str(AGREEMENTTERMORCOND_JSON).unwrap();

        assert_eq!(termorcond.description.as_str(), "Description");
        assert_eq!(termorcond.id.as_str(), "ATOC123");
    }

    #[test]
    fn test_termorcond_hasvalidity() {
        let mut termorcond = AgreementTermOrCondition::default();

        termorcond.set_validity(TimePeriod::period_30days());

        assert_eq!(termorcond.valid_for.is_some(), true);
    }

    #[test]
    fn test_agreementitemref_deserialize() {
        let agreementitemref: AgreementItemRef =
            serde_json::from_str(AGREEMENTITEMREF_JSON).unwrap();

        assert_eq!(agreementitemref.id.as_str(), "AI123");
        assert_eq!(
            agreementitemref.href.as_str(),
            "http://example.com/tmf651/item/AI123"
        );
        assert_eq!(agreementitemref.agreement_item_id.as_str(), "AI456");
        assert_eq!(agreementitemref.name.as_str(), "AgreementItem");
    }
}
