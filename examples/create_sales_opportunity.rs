//! Create Sales Opportunity
//!

#[cfg(all(feature = "tmf699", feature = "build-V5"))]
use tmflib::tmf699::sales_opportunity_item_v5::SalesOpportunityItem;
#[cfg(all(feature = "tmf699", feature = "build-V5"))]
use tmflib::tmf699::sales_opportunity_v5::SalesOpportunity;
use tmflib::tmf629::customer::Customer;
use tmflib::HasName;
fn main() {
    #[cfg(all(feature = "tmf696", feature = "build-V5"))]
    {
        let mut cust = Customer::default();
        cust.set_name("A Customer");

        let item = SalesOpportunityItem::new().for_customer(cust);

        let opp = SalesOpportunity::new("My Opportunity")
            .description("This is a new opportunity")
            .item(item);

        dbg!(opp);
    }
}
