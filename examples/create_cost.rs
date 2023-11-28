//! Create Cost Example

use tmflib::tmf7xx::cost::{CostEntry,Cost};

fn main() {
    let mut cost = Cost::new("ParentCost")
        .cost(CostEntry{unit: "Dollars".to_string(),amount: 237.5});
    let child_cost_1 = Cost::new("ChildCost")
        .cost(CostEntry{unit: "Dollars".to_string(),amount: 101.0});
    let child_cost_2 = Cost::new("Second Child")
        .cost(CostEntry{unit: "Dollars".to_string(),amount : 125.99});

    cost.add_child(child_cost_1);
    cost.add_child(child_cost_2);

    dbg!(&cost);

    println!("Total cost: {}",cost.total_cost())
}