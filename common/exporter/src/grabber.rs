use prometheus::proto::MetricFamily;

pub trait Grabber {
    fn name(&self) -> &str;
    fn help(&self) -> &str;
    fn collect(&self) -> Vec<MetricFamily>;
}
