use prometheus::{Gauge};
use prometheus::proto::{MetricFamily};
use prometheus::core::{Collector, Desc, Opts};

use std::time::Instant;

pub fn register_collector() {
    let metrics = vec![
        Gauge::new("request_duration_millis", "request duration millis").unwrap(),
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
        let mut metrics = Vec::new();
        
        let mut opts = Opts::new("request_duration_millis", "request duration millis");
        opts = opts.const_label("service", "https://www.rust-lang.org");
        let mut g = Gauge::with_opts(opts).unwrap();
        let mut start = Instant::now();
        let mut url = "https://www.rust-lang.org/";
        let mut res = reqwest::blocking::get(url);
        match res {
            Ok(resp) => {
                // println!("resp: {}", resp);
                println!("response: {} - {}", resp.status(), url);
                // println!("Headers:\n{:?}", resp.headers());

                // copy the response body directly to stdout
                // resp.copy_to(&mut std::io::stdout())?;

                g.set(start.elapsed().as_millis() as f64)
            },
            Err(e) => {
                println!("error: {}", e);
                g.set(0 as f64)
            }
        };

        metrics.extend(g.collect());


        opts = Opts::new("request_duration_millis", "request duration millis");
        opts = opts.const_label("service", "https://github.com");
        g = Gauge::with_opts(opts).unwrap();
        start = Instant::now();
        url = "https://github.com/";
        res = reqwest::blocking::get(url);
        match res {
            Ok(resp) => {
                // println!("resp: {}", resp);
                println!("response: {} - {}", resp.status(), url);
                // println!("Headers:\n{:?}", resp.headers());

                // copy the response body directly to stdout
                // resp.copy_to(&mut std::io::stdout())?;

                g.set(start.elapsed().subsec_millis() as f64)
            },
            Err(e) => {
                println!("error: {}", e);
                g.set(0 as f64)
            }
        };

        metrics.extend(g.collect());

        metrics
    }

}