//! Shopping Cart to Product Order
//! 

#![allow(unused_imports)]

use tmflib::HasRelatedParty;
use tmflib::common::related_party::RelatedParty;
use tmflib::common::note::Note;
#[cfg(all(feature = "tmf620", feature = "build-V4"))]
use tmflib::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
#[cfg(all(feature = "tmf620", feature = "build-V5"))]
use tmflib::tmf620::product_offering_v5::{ProductOffering,ProductOfferingRef};
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::individual_v5::Individual;
#[cfg(all(feature = "tmf663", feature = "build-V4"))]
use tmflib::tmf663::shopping_cart::ShoppingCart;
#[cfg(all(feature = "tmf663", feature = "build-V4"))]
use tmflib::tmf663::cart_item::CartItem;
#[cfg(all(feature = "tmf622", feature = "build-V4"))]
use tmflib::tmf622::product_order_v4::ProductOrder;
#[cfg(all(feature = "tmf622", feature = "build-V5"))]
use tmflib::tmf622::product_order_v5::ProductOrder;

fn main() {
    #[cfg(all(feature = "tmf663", feature = "build-V4"))]
    {
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
}