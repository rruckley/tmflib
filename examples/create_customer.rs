//! Create Customer example
//! 
use tmflib::tmf629::customer::Customer;
#[cfg(feature = "tmf632-v5")]
use tmflib::tmf632::organization_v5::Organization;

use tmflib::HasId;

fn main() {
    // let org = Organization::new("ACustomer");
    // let customer = Customer::new(org);

    // dbg!(&customer);
    let mut customer = Customer::default();
    customer.set_id("1");
    customer.generate_code(None);
    customer.generate_href();
    dbg!(&customer);

    customer.upgrade_to_code(None);
    dbg!(customer);

    // println!("Customer Code: {}",customer.get_characteristic("code").unwrap().value);
}