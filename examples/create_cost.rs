//! Create Example Cost

fn main() {
    #[cfg(feature = "tmf764")]
    {
        use tmflib::tmf764::actual_cost::{ActualCost,ActualCostItem};
        use tmflib::HasDescription;

        let item = ActualCostItem::default();
        let cost = ActualCost::new("Widget Deployment Cost")
            .description("Cost associated with deploying widgets to customer sites")
            .item(item);

        dbg!(cost);
    }
}
