//! Performance Management Example
//!

use tmflib::tmf628::performance_measurement::PerformanceMeasurement;
use tmflib::{HasDescription,HasId};

fn main() {
    let perf = PerformanceMeasurement::create().description("Example Performance Measurement");

    dbg!(perf);
}
