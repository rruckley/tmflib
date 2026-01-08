//! TMF676 Payment Module
//!

use crate::common::money::Money;
use crate::common::related_entity::EntityRef;
use crate::common::related_party::RelatedParty;
use crate::tmf666::AccountRef;
use crate::tmf676::PaymentMethodRefOrValue;
use crate::{vec_insert, DateTime, HasDescription, HasId, HasName, Uri};
use serde::{Deserialize, Serialize};

use tmflib_derive::{HasDescription, HasId, HasName};

use super::MOD_PATH;

const CLASS_PATH: &str = "payment";

/// Reference to another TMF schema
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct PaymentItem {
    amount: Option<Money>,
    id: Option<String>,
    tax_amount: Option<Money>,
    total_amount: Option<Money>,
    item: EntityRef,
}

impl PaymentItem {
    /// Create new Payment Item
    pub fn new(entity: impl HasName) -> PaymentItem {
        PaymentItem {
            item: entity.as_entity(),
            ..Default::default()
        }
    }

    /// Set the amount for this transaction
    pub fn amount(mut self, amount: f32) -> PaymentItem {
        self.amount = Some(Money::from(amount));
        self
    }

    /// Set the tax amount for this payment
    pub fn tax(mut self, tax: f32) -> PaymentItem {
        let tax = Money::from(tax);
        self.tax_amount = Some(tax.clone());
        if let Some(amount) = self.amount.clone() {
            self.total_amount = Some(amount + tax);
        };
        self
    }
}

/// A Payment
#[derive(Clone, Debug, Default, HasId, HasName, HasDescription, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    /// Payment Amount
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authorization_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correlation_id: Option<String>,
    /// Payment Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_date: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_amount: Option<Money>,
    /// Unique Id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// HTTP Uri
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Uri>,
    /// Refund Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // Referenced Objects
    payer: Option<RelatedParty>,
    payment_method: PaymentMethodRefOrValue,
    account: AccountRef,
    payment_item: Option<Vec<PaymentItem>>,
}

impl Payment {
    /// Create a new Payment from a payment method and account
    pub fn new(method: PaymentMethodRefOrValue, account: AccountRef) -> Payment {
        Payment {
            account,
            payment_method: method,
            ..Payment::create()
        }
    }

    /// Set the payer
    pub fn payer(mut self, party: impl Into<RelatedParty>) -> Payment {
        self.payer = Some(party.into());
        self
    }

    /// Add paymet item to the payment
    pub fn item(mut self, item: PaymentItem) -> Payment {
        vec_insert(&mut self.payment_item, item);
        self
    }

    /// Set the amount for this transaction
    pub fn amount(mut self, amount: f32) -> Payment {
        self.amount = Some(Money::from(amount));
        self
    }

    /// Set the tax amount for this payment
    pub fn tax(mut self, tax: f32) -> Payment {
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
    use crate::tmf632::individual_v4::Individual;

    #[test]
    fn test_payment_new() {
        let method = PaymentMethodRefOrValue::default().name("Credit Card");
        let account = AccountRef::default();
        let payment = Payment::new(method, account);
        assert!(payment.payment_item.is_none());
        assert!(payment.id.is_some());
        assert!(payment.href.is_some());
    }

    #[test]
    fn test_payment_payer() {
        let method = PaymentMethodRefOrValue::default().name("Credit Card");
        let account = AccountRef::default();
        let payer = Individual::new("John Quinton Smith");
        let payment = Payment::new(method, account).payer(&payer);

        assert!(payment.payer.is_some());
    }

    #[test]
    fn test_payment_item() {
        use crate::tmf632::individual_v4::Individual;
        use crate::tmf637::v4::product::Product;
        let method = PaymentMethodRefOrValue::default().name("Credit Card");
        let account = AccountRef::default();
        let payer = Individual::new("John Quinton Smith");
        let product1 = Product::new("Mobile Phone");
        let item1 = PaymentItem::new(product1).amount(100.0).tax(10.0);
        let payment = Payment::new(method, account).payer(&payer).item(item1);

        assert!(payment.payment_item.is_some());
        assert_eq!(payment.payment_item.unwrap().len(), 1);
    }

    #[test]
    fn test_payment_amount() {
        let method = PaymentMethodRefOrValue::default().name("Credit Card");
        let account = AccountRef::default();
        let payment = Payment::new(method, account).amount(100.0).tax(10.0);

        assert!(payment.amount.is_some());
        assert!(payment.tax_amount.is_some());
        assert!(payment.total_amount.is_some());
        assert_eq!(payment.amount.unwrap(), Money::from(100.0));
        assert_eq!(payment.tax_amount.unwrap(), Money::from(10.0));
        assert_eq!(payment.total_amount.unwrap(), Money::from(110.0));
    }
}
