#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prometheus;
use prometheus::{IntCounter, TextEncoder, Encoder};
mod collector;
pub use collector::register_collector;

use log::{debug};

lazy_static! {
    pub static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!("ratchet_high_five", "Number of high five received").unwrap();
    pub static ref NOT_FOUND_COUNTER: IntCounter =
        register_int_counter!("ratchet_not_found", "Not found").unwrap();
}

pub fn register() {
    register_collector();
}

/// Return all `MetricFamily` of registry
pub fn gather() -> String {
    debug!("prometheus.gather");
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();

    // Encode them to send.
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer.clone()).unwrap()
}
