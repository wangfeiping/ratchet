// use prometheus::Gauge;
// use prometheus::core::{Collector, Opts};

use prometheus::{Opts, Gauge};
use prometheus::proto::MetricFamily;
use prometheus::core::{Collector};

use exporter::Grabber;

// use std::time::Instant;

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub url: String,
}

pub fn get_services() -> Vec<Service> {
    let mut services = Vec::new();

    let mut srv = Service {
        name:String::from("rust-lang.org"),
        url: String::from("https://www.rust-lang.org")
    };
    services.push(srv);

    srv = Service {
        name:String::from("github.com"),
        url: String::from("https://github.com")
    };
    services.push(srv);

    services
}

struct Watcher {

}

impl Grabber for Watcher {
    fn name(&self) -> &str {
        "request_duration_millis"
    }
    fn help(&self) -> &str {
        "request duration millis"
    }
    fn collect(&self) -> Vec<MetricFamily> {
        let mut opts = Opts::new("request_test", "request test");
        opts = opts.const_label("service", "https://www.rust-lang.org");
        let m = Gauge::with_opts(opts).unwrap();
        m.inc();
        
        m.collect()
    }
}

pub fn get_handler() -> impl Grabber {
    // move || {
    //     let services = get_services();

    //     services
    //     .iter()
    //     .map(|srv| {
    //         let mut opts = Opts::new("request_duration_millis", "request duration millis");
    //         opts = opts.const_label("service", &srv.name);
    //         let g = Gauge::with_opts(opts).unwrap();
    //         let start = Instant::now();
    //         let res = reqwest::blocking::get(&srv.url);
    //         match res {
    //             Ok(resp) => {
    //                 // println!("resp: {}", resp);
    //                 println!("response: {} - {}", resp.status(), srv.url);
    //                 // println!("Headers:\n{:?}", resp.headers());
    
    //                 // copy the response body directly to stdout
    //                 // resp.copy_to(&mut std::io::stdout())?;
    
    //                 g.set(start.elapsed().as_millis() as f64)
    //             },
    //             Err(e) => {
    //                 println!("error: {}", e);
    //                 g.set(0 as f64)
    //             }
    //         };
    
    //         g.collect()
    //     })
    //     .fold(Vec::new(), |mut acc, mfs| {
    //         acc.extend(mfs);	
    //         acc
    //     })
    // }

    Watcher {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let srv = super::Service {
            name:String::from("test"),
            url: String::from("https://www.rust-lang.org")
        };

        assert_eq!(srv.name, "test");
    }
}
