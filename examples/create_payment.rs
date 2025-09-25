//! TMF676 Create Payment Example

#[cfg(feature = "tmf676")]
use tmflib::tmf676::payment::Payment;

use tmflib::tmf632::individual_v4::Individual;
use tmflib::tmf637::v4::product::Product;
use tmflib::HasDescription;
fn main() {
    #[cfg(feature = "tmf676")]
    {
        use tmflib::{tmf666::AccountRef, tmf676::{payment::PaymentItem, PaymentMethodRefOrValue}, HasName};

        let amount = 123.45;

        let product1 = Product::new("Mobile Phone");
        let product2 = Product::new("Phone Case");

        let party = Individual::new("John Quinton Smith");
        let method = PaymentMethodRefOrValue::default()
            .name("Credit Card");
        let account = AccountRef::default();
        let item1 = PaymentItem::new(product1)
            .amount(100.0);
        let item2 = PaymentItem::new(product2)
            .amount(23.45);

        let payment = Payment::new(method,account)
            .payer(&party)
            .description("Final Payment for device")
            .amount(amount)
            .tax(amount / 10.0)
            .item(item1)
            .item(item2);

        dbg!(payment);
    }
}