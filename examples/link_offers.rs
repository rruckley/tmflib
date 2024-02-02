//! Example to link Product Offers
//! # Description

#[cfg(feature = "tmf620-v4")]
use tmflib::tmf620::product_offering::ProductOffering;
#[cfg(feature = "tmf620-v5")]
use tmflib::tmf620::product_offering_v5::ProductOffering;

fn main() {
    let offer = ProductOffering::new("OriginalProduct");

    let mut new_offer = ProductOffering::new("NewProduct");

    new_offer.link_po(offer, "ParentChild", "Child");

    dbg!(new_offer);
}