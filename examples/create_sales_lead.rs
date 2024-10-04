//! Test Sales LEad

#[cfg(feature = "tmf699-v4")]
use tmflib::tmf699::sales_lead_v4::SalesLead;
#[cfg(feature = "tmf699-v5")]
use tmflib::tmf699::sales_lead_v5::SalesLead;

fn main() {
    #[cfg(feature = "tmf699-v4")]
    {
        let sl = SalesLead::new("My Sales Lead");

        dbg!(sl);
    
    }
}