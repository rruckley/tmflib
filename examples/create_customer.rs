//! Create Customer example
//! 
use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;

fn main() {
    let org = Organization::new(String::from("ACustomer"));
    let mut customer = Customer::new(org);

    customer.name(String::from("NewName"));

    dbg!(customer);
}