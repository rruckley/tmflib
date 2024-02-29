//! Communication Message Example

use tmflib::tmf681::communication_message::CommunicationMessage;


fn main() {

    let message = CommunicationMessage::email("A Subject","Some Content");

    dbg!(message);
}