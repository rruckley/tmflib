//! Create a service test
//! 

#[cfg(feature = "tmf653-v4")]
use tmflib::tmf653::service_test::ServiceTest;

fn main() {

    #[cfg(feature = "tmf653-v4")]
    {
        let st = ServiceTest::new("MyServiceTest");

        dbg!(st);
    
    }
}