use prometheus::{Gauge};
use prometheus::proto::{MetricFamily};
use prometheus::core::{Collector, Desc};

use std::time::{SystemTime, UNIX_EPOCH};

pub fn register_collector() {
    let metrics = vec![
        Gauge::new("request_duration_millis", "request duration millis").unwrap(),
    ];

    let descs = metrics.iter().map(|c| c.desc().into_iter().cloned()).fold(
        Vec::new(),
        |mut acc, ds| {
            acc.extend(ds);
            acc
        },
    );

    let rc = RatchetCollector { descs, metrics };

    prometheus::default_registry()
    .register(Box::new(rc)).unwrap();
}

pub struct RatchetCollector {
    descs: Vec<Desc>,
    metrics: Vec<Gauge>,
}

impl Collector for RatchetCollector {
    // Return descriptors for metrics.
    fn desc(&self) -> Vec<&Desc> {
        self.descs.iter().collect()
    }

    // Collect metrics.
    fn collect(&self) -> Vec<MetricFamily> {
        self.metrics
        .iter()
        .inspect(|c| {
            let start = SystemTime::now();
            let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
            let ms = since_the_epoch.subsec_millis() as f64;
            c.set(ms)
        })
        .map(|c| c.collect())
        .fold(Vec::new(), |mut acc, mfs| {
            acc.extend(mfs);
            acc
        })
    }

}