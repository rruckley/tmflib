//! Create Individual EXample
//! 
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::individual_v4::Individual;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::individual_v5::Individual;

fn main() {
    let individual = Individual::new("John Bagford Smith")
        .email("john.smith@example.com")
        .mobile("0411 111 111")
        .title("Mr")
        .preferred("Baggie");

    dbg!(individual);
}