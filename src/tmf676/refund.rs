//! TMF676 Refund Module

use crate::common::money::Money;
use crate::common::related_party::RelatedParty;
use crate::tmf666::AccountRef;
use crate::tmf676::PaymentMethodRefOrValue;
use crate::{DateTime, HasDescription, HasId, HasName, Uri};
use serde::{Deserialize, Serialize};

use tmflib_derive::{HasDescription, HasId, HasName};

use super::MOD_PATH;

const CLASS_PATH: &str = "refund";

/// A Refund
#[derive(Clone, Debug, Default, HasId, HasName, HasDescription, Serialize, Deserialize)]
pub struct Refund {
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP Uri
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Refund Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Amount of Refund
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<Money>,
    /// Authorization Code
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_code: Option<String>,
    /// Correlation Id
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<String>,
    /// Refund Description
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_amount: Option<Money>,
    // Referenced Types
    #[serde(skip_serializing_if = "Option::is_none")]
    requestor: Option<RelatedParty>,
    payment_method: PaymentMethodRefOrValue,
    account: AccountRef,
}

impl Refund {
    /// Create a new Refund object
    pub fn new(method: PaymentMethodRefOrValue, account: AccountRef) -> Refund {
        Refund {
            account,
            payment_method: method,
            ..Refund::create()
        }
    }

    /// Set the requestor
    pub fn requestor(mut self, party: impl Into<RelatedParty>) -> Refund {
        self.requestor = Some(party.into());
        self
    }

    /// Set the amount for this refund
    pub fn amount(mut self, amount: f32) -> Refund {
        self.amount = Some(Money::from(amount));
        self
    }

    /// Set the tax amount of this refund
    pub fn tax(mut self, tax: f32) -> Refund {
        let tax = Money::from(tax);
        self.tax_amount = Some(tax.clone());
        if let Some(amount) = self.amount.clone() {
            self.total_amount = Some(amount + tax);
        };
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_refund_new() {
        let method = PaymentMethodRefOrValue::default().name("Credit Card");
        let account = AccountRef::default();
        let refund = Refund::new(method, account);

        assert!(refund.id.is_some());
        assert!(refund.href.is_some());
    }

    #[test]
    fn test_refund_requestor() {
        use crate::tmf632::individual_v4::Individual;

        let method = PaymentMethodRefOrValue::default().name("Credit Card");
        let account = AccountRef::default();
        let party = Individual::new("John Quinton Smith");
        let refund = Refund::new(method, account).requestor(&party);

        assert!(refund.id.is_some());
        assert!(refund.href.is_some());
        assert!(refund.requestor.is_some());
    }

    #[test]
    fn test_refund_amount() {
        use crate::tmf632::individual_v4::Individual;

        let method = PaymentMethodRefOrValue::default().name("Credit Card");
        let account = AccountRef::default();
        let party = Individual::new("John Quinton Smith");
        let refund = Refund::new(method, account)
            .requestor(&party)
            .amount(100.0)
            .tax(10.0);

        assert!(refund.amount.is_some());
        assert!(refund.tax_amount.is_some());
        assert!(refund.total_amount.is_some());

        assert_eq!(refund.amount.unwrap(), Money::from(100.0));
        assert_eq!(refund.total_amount.unwrap(), Money::from(110.0));
    }
}
