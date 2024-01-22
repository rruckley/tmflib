//! Create an Agreement
//! 

use tmflib::tmf651::agreement::Agreement;

pub fn main() {
    let agreement = Agreement::new("My Agreement");

    dbg!(agreement);
}