//! Customer Event Creation
//!
 
use tmflib::common::event::EventPayload;
use tmflib::tmf632::organization::Organization;
use tmflib::tmf629::customer::{Customer,CustomerEventType};

fn main() {

    let org = Organization::new("An Organization");
    let cust = Customer::from(&org);

    let event = cust.to_event(CustomerEventType::CustomerCreateEvent)
        .path("status");

    dbg!(event);
}