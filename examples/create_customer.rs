//! Create Customer example
//!
use tmflib::tmf629::customer::Customer;

use tmflib::HasId;

fn main() {
    // let org = Organization::new("ACustomer");
    // let customer = Customer::new(org);

    // dbg!(&customer);
    let mut customer = Customer::default();
    customer.set_id("1");
    customer.generate_code(None);
    customer.generate_href();
    customer.set_market_segment("Health Industry");
    dbg!(&customer);

    customer.upgrade_to_code(None);
    dbg!(customer);
}
