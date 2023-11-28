//! Create Cost Example

use tmflib::tmf7xx::cost::Cost;

fn main() {
    let mut cost = Cost::new("MyCost");
    let child_cost = Cost::new("ChildCost");

    cost.add_child(child_cost);

    dbg!(&cost);

    println!("Total cost: {}",cost.total_cost())
}