use prometheus::{Gauge};
use prometheus::proto::{MetricFamily};
use prometheus::core::{Collector, Desc};

use std::time::Instant;

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
            let start = Instant::now();
            let url = "https://www.rust-lang.org/";
            let res = reqwest::blocking::get(url);

            match res {
                Ok(resp) => {
                    // println!("resp: {}", resp);
                    println!("response: {} - {}", resp.status(), url);
                    // println!("Headers:\n{:?}", resp.headers());

                    // copy the response body directly to stdout
                    // resp.copy_to(&mut std::io::stdout())?;

                    c.set(start.elapsed().subsec_millis() as f64)
                },
                Err(e) => {
                    println!("error: {}", e);
                    c.set(0 as f64)
                }
            };

            ()
        })
        .map(|c| c.collect())
        .fold(Vec::new(), |mut acc, mfs| {
            acc.extend(mfs);
            acc
        })
    }

}