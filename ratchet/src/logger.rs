use log::{error};
use log4rs;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

fn init_default() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} -{t} - {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder()
            .appender("stdout")
            .additive(false)
            .build("app", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

pub fn init_logger() {
    let ret = log4rs::init_file(
        "./log4rs.yaml", Default::default());
    match ret {
        Ok(ret) => ret,
        Err(e) => {
            init_default();
            error!("{}", e)
        }
    };
}