//! Create Sales Opportunity
//! 

use tmflib::tmf699::sales_opportunity_v5::SalesOpportunity;
use tmflib::tmf699::sales_opportunity_item_v5::SalesOpportunityItem;
use tmflib::tmf629::customer::Customer;
use tmflib::HasName;

fn main() {
    let mut cust = Customer::default();
    cust.set_name("A Customer");
    let item= SalesOpportunityItem::new()
        .for_customer(cust);    
    let opp = SalesOpportunity::new("My Opportunity")
        .description("This is a new opportunity")
        .item(item);

    dbg!(opp);
}