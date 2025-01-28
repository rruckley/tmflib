//! Create Ticket Example

#[cfg(feature = "tmf621")]
use tmflib::{tmf621::trouble_ticket::TroubleTicket, HasDescription};

fn main() {
    #[cfg(feature = "tmf621")]
    {
        let ticket = TroubleTicket::new("A Ticket")
        .description("This is a trouble ticket");
    
    
        dbg!(ticket);
    }
}