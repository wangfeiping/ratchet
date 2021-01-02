use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use ratchet_version::VERSION;
use clap::{App, Arg, ArgMatches};
use log::{error, info, warn};
use log4rs;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use std::process::exit;

const CRLF: &str = "\r\n";

fn init_log() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[Console] {d} - {l} -{t} - {m}{n}")))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[File] {d} - {l} - {t} - {m}{n}")))
        .build("log/test.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(Logger::builder()
            .appender("file")
            .additive(false)
            .build("app", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();
}

fn main() {
    init_log();

    // Parse the CLI parameters.
    let matches = App::new("Ratchet")
        .version(VERSION.replace("Ratchet/", "").as_str())
        .author("wangfeiping <wangfeiping@outlook.com>")
        .setting(clap::AppSettings::ColoredHelp)
        .about(
            "net watcher \
             developed using rust",
        )
        .long_version(
            format!(
                "{}\n\
                 Rust 2018",
                VERSION.replace("Ratchet/", ""),
            ).as_str()
        )
        .arg(
            Arg::with_name("log-level")
                .long("log-level")
                .value_name("LEVEL")
                .help("The verbosity level for emitting logs.")
                .takes_value(true)
                .possible_values(&["info", "debug", "trace", "warn", "error", "crit"])
                .global(true)
                .default_value("info"),

        )
        .get_matches();

    let result = run(&matches);

    // `std::process::exit` does not run destructors so we drop manually.
    drop(matches);

    // Return the appropriate error code.
    match result {
        Ok(()) => exit(0),
        Err(e) => {
            error!("{}", e);
            drop(e);
            exit(1)
        }
    }
}

fn run(
    matches: &ArgMatches,
) -> Result<(), String> {
    let log_level = matches
        .value_of("log-level")
        .ok_or("Expected --log-level flag")?;

    println!("log-level: {}", log_level);
    info!("booting up");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| handle_connection(stream));
    }
    
    Ok(())
}

fn handle_index() -> (String, String) {
    let mut contents= String::new();
    contents.push_str("Hello!");
    contents.push_str(
        format!(
            " {}\n", VERSION.replace("Ratchet/", "")
        ).as_str());
    info!("{}", contents);
    (contents, status(200, "OK"))
}

fn handle_404() -> (String, String) {
    let msg = "404 Not Found!";
    warn!("{}", msg);
    (msg.to_string(), status(404, "OK"))
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer= [0; 4096];
    stream.read(&mut buffer).unwrap();

    let _matched = |route: &str| matched(&buffer, route);
    let _write = |(contents, status)| write(stream, contents, status);

    if _matched("/") {
        _write(handle_index());
    } else {
        _write(handle_404());
    }
}

fn matched(buffer: &[u8; 4096], route: &str) -> bool {
    let s = format!("GET {} HTTP/1.1{}", route, CRLF);
    buffer.starts_with(s.as_bytes())
}

fn status(code: i32, text: &str) -> String {
    format!("HTTP/1.1 {} {}{}", code, text, CRLF)
}

fn write(mut stream: TcpStream, contents: String, status: String) {
    let content_type = format!("Content-Type: text/html;charset=utf-8{}", CRLF);
    let server = format!("Server: Rust{}", CRLF);
    let content_length = format!("Content-Length: {}{}", contents.as_bytes().len(), CRLF);
    let response = format!(
        "{0}{1}{2}{3}{4}{5}",
        status, server, content_type, content_length, CRLF, contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

