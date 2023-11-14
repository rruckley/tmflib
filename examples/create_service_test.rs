//! Create a service test
//! 

use tmflib::tmf653::service_test::ServiceTest;

fn main() {
    let st = ServiceTest::new("MyServiceTest");

    dbg!(st);
}