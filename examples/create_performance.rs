//! Performance Management Example
//!

use tmflib::tmf628::PerformanceMeasurement;
use tmflib::{HasDescription, HasId};

fn main() {
    let perf = PerformanceMeasurement::create().description("Example Performance Measurement");

    dbg!(perf);
}
