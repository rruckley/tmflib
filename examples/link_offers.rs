//! Example to link Product Offers
//! # Description

#[cfg(all(feature = "tmf620", feature = "build-V4"))]
use tmflib::tmf620::product_offering::ProductOffering;
#[cfg(all(feature = "tmf620", feature = "build-V5"))]
use tmflib::tmf620::product_offering_v5::ProductOffering;

fn main() {
    let offer = ProductOffering::new("OriginalProduct");

    let mut new_offer = ProductOffering::new("NewProduct");

    new_offer.link_po(offer, "ParentChild", "Child");

    dbg!(new_offer);
}