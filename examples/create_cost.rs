//! Create Cost Example

use tmflib::tmf7xx::cost::Cost;
use tmflib::common::money::Money;

fn main() {
    let mut cost = Cost::new("ParentCost")
        .cost(Money{unit: "Dollars".to_string(),value: 237.5});
    let child_cost_1 = Cost::new("ChildCost")
        .cost(Money{unit: "Dollars".to_string(),value: 101.0});
    let child_cost_2 = Cost::new("Second Child")
        .cost(Money{unit: "Dollars".to_string(),value : 125.99});

    cost.add_child(child_cost_1);
    cost.add_child(child_cost_2);

    dbg!(&cost);

    println!("Total cost: {}",cost.total_cost())
}