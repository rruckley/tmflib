//! Appointment booking example

#[cfg(feature = "tmf646-v4")]
use tmflib::tmf646::appointment::Appointment;

fn main() {
    #[cfg(feature = "tmf646-v4")]
    {
        let appointment = Appointment::new();

        dbg!(appointment);
    }
}