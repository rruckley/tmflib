//! Shopping Cart Example

use tmflib::common::related_party::RelatedParty;
use tmflib::common::note::Note;
use tmflib::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
use tmflib::tmf632::individual::Individual;
use tmflib::tmf663::shopping_cart::ShoppingCart;
use tmflib::tmf663::cart_item::CartItem;


fn main() {
    let mut cart = ShoppingCart::new();
    let offer = ProductOffering::new(String::from("MyProductOffer"));
    let por = ProductOfferingRef::from(offer);
    let individual = Individual::new("John Smith".to_string())
        .email("john.smith@example.com")
        .mobile("0411 111 111");
    let note1 = Note::from("Checking on stock levels");

    let mut item = CartItem::from(por);
    item.add_note(note1);
    cart.add_item(item);
    cart.add_party(RelatedParty::from(&individual));

    dbg!(cart);
}