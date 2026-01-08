//! Customer  Bill Example
//!

#[cfg(all(feature = "tmf678", feature = "build-V4"))]
use tmflib::tmf678::customer_bill_v4::CustomerBill;
#[cfg(all(feature = "tmf678", feature = "build-V5"))]
use tmflib::tmf678::customer_bill_v5::CustomerBill;

fn main() {
    #[cfg(feature = "tmf678")]
    {
        let bill = CustomerBill::new();

        dbg!(bill);
    }
}
