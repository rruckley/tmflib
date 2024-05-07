//! Customer Event Creation
//!
 
use tmflib::common::event::EventPayload;
#[cfg(feature = "tmf632-v4")]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(feature = "tmf632-v5")]
use tmflib::tmf632::organization_v5::Organization;
use tmflib::tmf629::customer::{Customer,CustomerEventType};

fn main() {

    let org = Organization::new("An Organization");
    let cust = Customer::from(&org);

    let event = cust.to_event(CustomerEventType::CustomerCreateEvent)
        .path("status");

    dbg!(event);
}