use log::{error};
use log4rs;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

fn init_default(level: LevelFilter) {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l} -{t} - {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder()
            .appender("stdout")
            .additive(false)
            .build("app", level))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();

    log4rs::init_config(config).unwrap();
}

pub fn init_logger(log_level: String) {
    // println!("log_level: {}", log_level);

    let level = match log_level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "crit" => LevelFilter::Error,
        _ => unreachable!(),
    };

    let ret = log4rs::init_file(
        "./log4rs.yaml", Default::default());
    match ret {
        Ok(ret) => ret,
        Err(e) => {
            init_default(level);
            error!("{}", e)
        }
    };
}