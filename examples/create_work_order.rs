//! Create Work Order Example

use tmflib::tmf697::v4::work_order::WorkOrder;
use tmflib::tmf697::v4::work::{
    Work,
    WorkRefOrValue,
};
use tmflib::tmf697::v4::work_order_item::WorkOrderItem;

fn main() {

    let mut wo = WorkOrder::new();

    let work = Work::new("Some work");

    let woi = WorkOrderItem::with(WorkRefOrValue::from(work));

    wo.add_item(woi);

    // dbg!(wo);
    let json = serde_json::to_string(&wo).unwrap();
    println!("JSON>>\n{}\n<<JSON",json);
}