//! Appointment booking example

use tmflib::tmf646::appointment::Appointment;

fn main() {
    let appointment = Appointment::new();

    dbg!(appointment);
}