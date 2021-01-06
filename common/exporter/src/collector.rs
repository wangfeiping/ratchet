use prometheus::{Gauge, TextEncoder, Encoder};
use prometheus::proto::{MetricFamily};
use prometheus::core::{Collector, Desc};

use log::{debug};
use std::time::Instant;

use crate::grabber::Grabber;

pub fn register(grabber: Box<dyn Grabber>)
{
    let metrics = vec![
        Gauge::new(grabber.name(), grabber.help()).unwrap(),
    ];

    let descs = metrics
    .iter()
    .map(|c| c.desc().into_iter().cloned())
    .fold(
        Vec::new(),
        |mut acc, ds| {
            acc.extend(ds);
            acc
        },
    );

    let rc = RatchetCollector { descs, g: grabber };

    prometheus::default_registry()
    .register(Box::new(rc)).unwrap();
}

/// Return all `metrics data` of registry
pub fn gather() -> String {
    let start = Instant::now();
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();

    // Encode them to send.
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let ret = String::from_utf8(buffer.clone());
    debug!("prometheus.gather() cost: {}", start.elapsed().as_millis());

    ret.unwrap()
}

struct RatchetCollector {
    descs: Vec<Desc>,
    g: Box<dyn Grabber>,
}

impl Collector for RatchetCollector {
    // Return descriptors for metrics.
    fn desc(&self) -> Vec<&Desc> {
        self.descs.iter().collect()
    }

    // Collect metrics.
    fn collect(&self) -> Vec<MetricFamily> {
        self.g.collect()
    }

}