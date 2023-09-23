//! Create a product template
//! 
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::category::{Category,CategoryRef};

fn main() {
    let offer = ProductOffering::new(String::from("ProductOffering"));
    let category = Category::new(String::from("Category"));
    let catref = CategoryRef::from(&category);
    offer.with_category(CategoryRef::from(&category));

    dbg!(catref);
}