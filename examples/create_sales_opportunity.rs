//! Create Sales Opportunity
//! 

use tmflib::tmf699::sales_opportunity_v5::SalesOpportunity;

fn main() {
    let opp = SalesOpportunity::new("My Opportunity")
        .description("This is a new opportunity");

    dbg!(opp);
}