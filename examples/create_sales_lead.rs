//! Test Sales LEad

#[cfg(all(feature = "tmf699", feature = "build-V4"))]
use tmflib::tmf699::sales_lead_v4::SalesLead;
#[cfg(all(feature = "tmf699", feature = "build-V5"))]
use tmflib::tmf699::sales_lead_v5::SalesLead;

fn main() {
    #[cfg(all(feature = "tmf699", feature = "build-V4"))]
    {
        let sl = SalesLead::new("My Sales Lead");

        dbg!(sl);
    }
}
