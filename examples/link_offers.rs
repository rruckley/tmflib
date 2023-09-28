//! Example to link Product Offers
//! 

use tmflib::tmf620::product_offering::{ProductOffering,ProductOfferingRelationship};

fn main() {
    let offer = ProductOffering::new(String::from("OriginalProduct"));

    let mut offer_rel = ProductOfferingRelationship::from(offer);
    offer_rel.relationship_type = Some(String::from("ParentChild"));
    offer_rel.role = Some(String::from("parent"));

    let mut new_offer = ProductOffering::new(String::from("NewProduct"));
    new_offer.product_offering_relationship.as_mut().unwrap().push(offer_rel);

    dbg!(new_offer);
}