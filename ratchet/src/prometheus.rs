use prometheus::{IntCounter, TextEncoder, Encoder};
use log::{debug};

lazy_static! {
    pub static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!("high_five", "Number of high five received").unwrap();
    pub static ref NOT_FOUND_COUNTER: IntCounter =
        register_int_counter!("not_found", "Not found").unwrap();
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
