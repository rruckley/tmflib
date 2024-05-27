//! Create Work Order Example

use tmflib::tmf697::v4::work_order::WorkOrder;

fn main() {

    let wo = WorkOrder::new();

    dbg!(wo);
}