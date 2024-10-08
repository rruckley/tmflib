//! Create a service test
//! 

#[cfg(all(feature = "tmf653", feature = "build-V4"))]
use tmflib::tmf653::service_test::ServiceTest;

fn main() {

    #[cfg(all(feature = "tmf653", feature = "build-V4"))]
    {
        let st = ServiceTest::new("MyServiceTest");

        dbg!(st);
    
    }
}