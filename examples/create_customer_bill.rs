//! Customer  Bill Example
//! 


use tmflib::tmf678::customer_bill_v4::CustomerBill;

fn main() {

    let bill = CustomerBill::new();

    dbg!(bill);
}