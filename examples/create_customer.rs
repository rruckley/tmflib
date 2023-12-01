//! Create Customer example
//! 
use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;

fn main() {
    let org = Organization::new(String::from("ACustomer"));
    let customer = Customer::new(org);

    dbg!(&customer);

    println!("Customer Code: {}",customer.get_characteristic("code").unwrap().value);
}