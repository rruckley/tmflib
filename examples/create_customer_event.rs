//! Customer Event Creation
//!

use tmflib::common::event::EventPayload;
use tmflib::tmf629::customer::{Customer, CustomerEventType};
#[cfg(all(feature = "tmf632", feature = "build-V4"))]
use tmflib::tmf632::organization_v4::Organization;
#[cfg(all(feature = "tmf632", feature = "build-V5"))]
use tmflib::tmf632::organization_v5::Organization;

fn main() {
    let org = Organization::new("An Organization");
    let cust = Customer::from(&org);

    let event = cust
        .to_event(CustomerEventType::CustomerCreateEvent)
        .path("status");

    dbg!(event);
}
