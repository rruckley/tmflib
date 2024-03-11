//! Communication Message Example

use tmflib::tmf681::communication_message::CommunicationMessage;
use tmflib::tmf632::individual::Individual;


fn main() {
    let from = Individual::new("John Smith");
    let to1 = Individual::new("Suzy Citizen");
    let to2 = Individual::new("Ryan Ruckley")
        .email("rruckley@gmail.com");

    let message = CommunicationMessage::email("A Subject","Some Content")
        .from(&from)
        .to(vec![&to1,&to2]);

    dbg!(message);
}