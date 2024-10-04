//! Create Work Order Example

#[cfg(feature = "tmf697-v4")]
use tmflib::tmf697::v4::work_order::WorkOrder;
#[cfg(feature = "tmf697-v4")]
use tmflib::tmf697::v4::work::{
    Work,
    WorkRefOrValue,
};
#[cfg(feature = "tmf697-v4")]
use tmflib::tmf697::v4::work_order_item::WorkOrderItem;

#[cfg(feature = "tmf697-v5")]
use tmflib::tmf697::v5::work_order_item::WorkOrderItem;
#[cfg(feature = "tmf697-v5")]
use tmflib::tmf697::v5::work_order::WorkOrder;
#[cfg(feature = "tmf697-v5")]
use tmflib::tmf697::v5::work::{
    Work,
    WorkRefOrValue,
};

fn main() {

    #[cfg(feature = "tmf697-v4")]
    {
        let mut wo = WorkOrder::new();

        let work = Work::new("Some work");
    
        let woi = WorkOrderItem::with(WorkRefOrValue::from(work));
    
        wo.add_item(woi);
    
        dbg!(wo);
    
    }

}