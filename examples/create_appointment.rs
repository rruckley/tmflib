//! Appointment booking example

#[cfg(all(feature = "tmf646", feature = "build-V4"))]
use tmflib::tmf646::appointment::Appointment;

fn main() {
    #[cfg(all(feature = "tmf646", feature = "build-V4"))]
    {
        let appointment = Appointment::new();

        dbg!(appointment);
    }
}
