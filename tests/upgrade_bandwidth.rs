//! Test to demonstrate generating a TMF641 order to increase band
//! on a service using data fetched by TMF638


#[test]
fn test_service_bandwidth_upgrade() {
    use tmflib::tmf638::service::{Characteristic,Service};
    use tmflib::tmf641::service_order::ServiceOrder;

    // // Create a new service
    // let mut service = Service::new("TestService")
    //     .with_characteristic(Characteristic  {
    //         name: "bandwidth".to_string(), 
    //         value: "100".into(),
    //         ..Default::default()
    // });


    // // Step 1 get current bandwidth
    // let current_bandwidth = service.get_characteristics("bandwidth");

    // let service_order = 
    // // Check if the event is generated correctly
    // assert_eq!(event.domain.unwrap(), Service::get_class());
}   