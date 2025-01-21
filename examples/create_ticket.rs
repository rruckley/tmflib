//! Create Ticket Example

use tmflib::{tmf621::trouble_ticket::TroubleTicket, HasDescription};

fn main() {
    let ticket = TroubleTicket::new("A Ticket")
    .description("This is a trouble ticket");


    dbg!(ticket);
}