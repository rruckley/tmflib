//! Example - Create Billing Account
//!

use tmflib::tmf666::billing_account::BillingAccount;

fn main() {
    let account = BillingAccount::new("MyBillingAccount");

    dbg!(account);
}
