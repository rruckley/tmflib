//! Create Work Order Example

#[cfg(all(feature = "tmf697", feature = "build-V4"))]
use tmflib::tmf697::v4::work::{Work, WorkRefOrValue};
#[cfg(all(feature = "tmf697", feature = "build-V4"))]
use tmflib::tmf697::v4::work_order::WorkOrder;
#[cfg(all(feature = "tmf697", feature = "build-V4"))]
use tmflib::tmf697::v4::work_order_item::WorkOrderItem;

#[cfg(all(feature = "tmf697", feature = "build-V5"))]
use tmflib::tmf697::v5::work::{Work, WorkRefOrValue};
#[cfg(all(feature = "tmf697", feature = "build-V5"))]
use tmflib::tmf697::v5::work_order::WorkOrder;
#[cfg(all(feature = "tmf697", feature = "build-V5"))]
use tmflib::tmf697::v5::work_order_item::WorkOrderItem;

fn main() {
    #[cfg(all(feature = "tmf697", feature = "build-V4"))]
    {
        let mut wo = WorkOrder::new();

        let work = Work::new("Some work");

        let woi = WorkOrderItem::with(WorkRefOrValue::from(work));

        wo.add_item(woi);

        dbg!(wo);
    }
}
