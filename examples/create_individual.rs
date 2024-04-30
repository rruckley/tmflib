//! Create Individual EXample
//! 
#[cfg(feature = "tmf632-v4")]
use tmflib::tmf632::individual_v4::Individual;

fn main() {
    let individual = Individual::new("John Bagford Smith")
        .email("john.smith@example.com")
        .mobile("0411 111 111")
        .title("Mr")
        .preferred("Baggie");

    dbg!(individual);
}