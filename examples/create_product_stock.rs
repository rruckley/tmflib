//! Create Product Stock

#[cfg(all(feature = "tmf687", feature = "build-V4"))]
use tmflib::tmf687::product_stock::ProductStock;

fn main() {

    #[cfg(all(feature = "tmf687", feature = "build-V4"))]
    {
        let stock = ProductStock::new("NTU Stock");

        dbg!(stock);    
    }
}