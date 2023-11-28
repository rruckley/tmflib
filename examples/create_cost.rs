//! Create Cost Example

use tmflib::tmf7xx::cost::{CostEntry,Cost};

fn main() {
    let mut cost = Cost::new("MyCost");
    let child_cost = Cost::new("ChildCost")
        .cost(CostEntry{unit: "Dollars".to_string(),amount: 10.0});

    cost.add_child(child_cost);

    dbg!(&cost);

    println!("Total cost: {}",cost.total_cost())
}