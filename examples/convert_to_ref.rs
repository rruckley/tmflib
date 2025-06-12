/// Example to show use of trait HasReference to convert a value to a reference.

use tmflib::tmf620::category::Category;
use tmflib::tmf620::catalog::Catalog;
use tmflib::tmf632::individual_v4::Individual;
use tmflib::HasReference;

fn main() {
    // Create a value of type i32
    let cat = Category::new( "Example Category");
    let catalog = Catalog::new("Example Catalog");
    let individual = Individual::new("Example Individual");

    let cat_ref = cat.as_ref();
    let catalog_ref = catalog.as_ref();
    let individual_ref = individual.as_ref();
    
    dbg!(cat_ref);
    dbg!(catalog_ref);
    dbg!(individual_ref);
}