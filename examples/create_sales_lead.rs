//! Test Sales LEad


use tmflib::tmf699::sales_lead::SalesLead;

fn main() {
    let sl = SalesLead::new("My Sales Lead");

    dbg!(sl);
}