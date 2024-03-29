//! Create Individual EXample
//! 
use tmflib::tmf632::individual::Individual;

fn main() {
    let individual = Individual::new("John Bagford Smith")
        .email("john.smith@example.com")
        .mobile("0411 111 111")
        .title("Mr")
        .preferred("Baggie");

    dbg!(individual);
}