//! Shopping Cart Example

#![allow(unused_imports)]

use tmflib::common::related_party::RelatedParty;
use tmflib::common::note::Note;
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf620::product_offering_v5::{ProductOffering,ProductOfferingRef};
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::individual_v5::Individual;
#[cfg(all(feature = "tmf663", feature = "build-V4"))]
use tmflib::tmf663::shopping_cart::ShoppingCart;
#[cfg(all(feature = "tmf663", feature = "build-V4"))]
use tmflib::tmf663::cart_item::CartItem;
use tmflib::HasRelatedParty;


fn main() {
    #[cfg(all(feature = "tmf663", feature = "build-V4"))]
    {
        let mut cart = ShoppingCart::new();
        let offer = ProductOffering::new("MyProductOffer");
        let por = ProductOfferingRef::from(offer);
        let individual = Individual::new("John Smith")
            .email("john.smith@example.com")
            .mobile("0411 111 111");
        let note1 = Note::from("Checking on stock levels");
    
        let mut item = CartItem::from(por);
        item.add_note(note1);
        cart.add_item(item);
        cart.add_party(RelatedParty::from(&individual));
    
        dbg!(cart);
    }

}