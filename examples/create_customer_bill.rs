//! Customer  Bill Example
//! 


#[cfg(feature = "tmf678-v4")]
use tmflib::tmf678::customer_bill_v4::CustomerBill;
#[cfg(feature = "tmf678-v5")]
use tmflib::tmf678::customer_bill_v5::CustomerBill;

fn main() {

    let bill = CustomerBill::new();

    dbg!(bill);
}