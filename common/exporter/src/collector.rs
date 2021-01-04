use prometheus::{Gauge};
use prometheus::proto::{MetricFamily};
use prometheus::core::{Collector, Desc};

pub fn register_collector<F>(f: F)
where
    F: Fn() -> Vec<MetricFamily>
{
    let metrics = vec![
        Gauge::new("request_duration_millis", "request duration millis").unwrap(),
    ];

    let _ff = f;
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

    let rc = RatchetCollector { descs };

    prometheus::default_registry()
    .register(Box::new(rc)).unwrap();
}

pub struct RatchetCollector {
    descs: Vec<Desc>,
}

impl Collector for RatchetCollector {
    // Return descriptors for metrics.
    fn desc(&self) -> Vec<&Desc> {
        self.descs.iter().collect()
    }

    // Collect metrics.
    fn collect(&self) -> Vec<MetricFamily> {
        Vec::new()
    }

}