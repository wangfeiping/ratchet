#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prometheus;
use prometheus::IntCounter;

mod collector;
mod grabber;

pub use grabber::Grabber;
pub use collector::{register, gather};

lazy_static! {
    pub static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!(opts!(
            "ratchet_high_five",
            "Number of high five received",
            labels!{"service" => "/", "foo" => "bar",})).unwrap();
    pub static ref NOT_FOUND_COUNTER: IntCounter =
        register_int_counter!("ratchet_not_found", "Not found").unwrap();
}
