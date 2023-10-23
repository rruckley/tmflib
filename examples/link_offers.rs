//! Example to link Product Offers
//! # Description

use tmflib::tmf620::product_offering::ProductOffering;

fn main() {
    let offer = ProductOffering::new(String::from("OriginalProduct"));

    let mut new_offer = ProductOffering::new(String::from("NewProduct"));

    new_offer.link_po(offer, "ParentChild", "Child");

    dbg!(new_offer);
}