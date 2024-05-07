//! Create Customer example
//! 
use tmflib::tmf629::customer::Customer;
#[cfg(feature = "tmf632-v4")]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(feature = "tmf632-v5")]
use tmflib::tmf632::organization_v5::Organization;

fn main() {
    let org = Organization::new("ACustomer");
    let customer = Customer::new(org);

    dbg!(&customer);

    println!("Customer Code: {}",customer.get_characteristic("code").unwrap().value);
}