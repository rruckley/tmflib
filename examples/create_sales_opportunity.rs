//! Create Sales Opportunity
//! 

#[cfg(feature ="tmf699-v5")]
use tmflib::tmf699::sales_opportunity_v5::SalesOpportunity;
#[cfg(feature = "tmf699-v5")]
use tmflib::tmf699::sales_opportunity_item_v5::SalesOpportunityItem;
use tmflib::tmf629::customer::Customer;
use tmflib::HasName;

fn main() {
    let mut cust = Customer::default();
    cust.set_name("A Customer");
    #[cfg(feature ="tmf699-v5")]
    let item= SalesOpportunityItem::new()
        .for_customer(cust);
    #[cfg(feature ="tmf699-v5")]
    let opp = SalesOpportunity::new("My Opportunity")
        .description("This is a new opportunity")
        .item(item);

    #[cfg(feature ="tmf699-v5")]
    dbg!(opp);
}