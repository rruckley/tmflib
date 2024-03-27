//! Shopping Cart to Product Order
use tmflib::common::related_party::RelatedParty;
use tmflib::common::note::Note;
#[cfg(feature = "tmf620-v4")]
use tmflib::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
#[cfg(feature = "tmf620-v5")]
use tmflib::tmf620::product_offering_v5::{ProductOffering,ProductOfferingRef};
use tmflib::tmf632::individual::Individual;
use tmflib::tmf663::shopping_cart::ShoppingCart;
use tmflib::tmf663::cart_item::CartItem;
use tmflib::tmf622::product_order_v4::ProductOrder;

fn main() {
    let mut cart = ShoppingCart::new();
    let offer = ProductOffering::new("MyProductOffer");
    let individual = Individual::new("John Smith")
        .email("john.smith@example.com")
        .mobile("0411 111 111");
    let note1 = Note::from("Checking on stock levels");

    let mut item = CartItem::from(ProductOfferingRef::from(offer));
    item.add_note(note1);
    item.quantity = 11;
    cart.add_item(item);
    cart.add_party(RelatedParty::from(&individual));

    let order = ProductOrder::from(cart);
    
    dbg!(order);
}