use prometheus::proto::MetricFamily;

pub trait Grabber: Sync + Send {
    fn name(&self) -> &str;
    fn help(&self) -> &str;
    fn collect(&self) -> Vec<MetricFamily>;
}
