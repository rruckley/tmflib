//! Shopping Cart Example

use tmflib::tmf620::product_offering::{ProductOffering,ProductOfferingRef};
use tmflib::tmf663::shopping_cart::ShoppingCart;
use tmflib::tmf663::cart_item::CartItem;

fn main() {
    let mut cart = ShoppingCart::new();
    let offer = ProductOffering::new(String::from("MyProductOffer"));
    let por = ProductOfferingRef::from(offer);
    cart.add_item(CartItem::from(por));

    dbg!(cart);
}