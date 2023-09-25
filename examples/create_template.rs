//! Create a product template
//! 
use tmflib::tmf620::product_offering::ProductOffering;
use tmflib::tmf620::category::{Category,CategoryRef};

fn main() {
    let category = Category::new(String::from("Template"));
    let offer = ProductOffering::new(String::from("ProductOffering"))
        .with_category(CategoryRef::from(&category));

    dbg!(&offer);
}