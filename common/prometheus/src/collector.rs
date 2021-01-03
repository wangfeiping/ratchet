use prometheus::{Counter};
use prometheus::proto::{MetricFamily};
use prometheus::core::{Collector, Desc};

pub fn register_collector() {
    let counters = vec![
        Counter::new("c1", "c1 is a counter").unwrap(),
        Counter::new("c2", "c2 is a counter").unwrap(),
    ];

    let descs = counters.iter().map(|c| c.desc().into_iter().cloned()).fold(
        Vec::new(),
        |mut acc, ds| {
            acc.extend(ds);
            acc
        },
    );

    let rc = RatchetCollector { descs, counters };

    prometheus::default_registry()
    .register(Box::new(rc)).unwrap();
}

pub struct RatchetCollector {
    descs: Vec<Desc>,
    counters: Vec<Counter>,
}

impl Collector for RatchetCollector {
    // Return descriptors for metrics.
    fn desc(&self) -> Vec<&Desc> {
        self.descs.iter().collect()
    }

    // Collect metrics.
    fn collect(&self) -> Vec<MetricFamily> {
        self.counters
        .iter()
        .inspect(|c| c.inc())
        .map(|c| c.collect())
        .fold(Vec::new(), |mut acc, mfs| {
            acc.extend(mfs);
            acc
        })
    }

}