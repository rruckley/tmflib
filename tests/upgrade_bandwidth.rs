//! Test to demonstrate generating a TMF641 order to increase band
//! on a service using data fetched by TMF638

#[test]
fn test_service_bandwidth_upgrade() {
    use tmflib::tmf638::service::{Characteristic, Service};
    // use tmflib::tmf641::service_order::ServiceOrder;

    // Create a new service
    let service = Service::new("TestService")
        .with_characteristic(Characteristic::new("bandwidth".to_string(), 100.into()));

    // // Step 1 get current bandwidth
    let current_bandwidth = service.get_characteristics("bandwidth");

    assert_eq!(current_bandwidth.is_some(), true);
    assert_eq!(current_bandwidth.unwrap().len(),1);


    // let service_order =
    // // Check if the event is generated correctly
    // assert_eq!(event.domain.unwrap(), Service::get_class());
}
