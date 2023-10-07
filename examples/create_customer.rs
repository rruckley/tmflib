//! Create Customer example
//! 
use tmflib::tmf629::customer::Customer;

fn main() {
    let mut customer = Customer::new(String::from("A Customer"));

    customer.name(String::from("NewName"));

    dbg!(customer);
}