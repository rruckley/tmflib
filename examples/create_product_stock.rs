//! Create Product Stock

#[cfg(feature = "tmf687-v4")]
use tmflib::tmf687::product_stock::ProductStock;

fn main() {

    #[cfg(feature = "tmf687-v4")]
    {
        let stock = ProductStock::new("NTU Stock");

        dbg!(stock);    
    }
}