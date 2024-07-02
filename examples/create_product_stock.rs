//! Create Product Stock

use tmflib::tmf687::product_stock::ProductStock;

fn main() {

    let stock = ProductStock::new("NTU Stock");

    dbg!(stock);
}