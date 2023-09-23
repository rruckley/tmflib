//! Create Customer example
//! 
use tmflib::tmf629::customer::Customer;

fn main() {
    let customer = Customer::new(String::from("A Customer"));

    dbg!(customer);
}