use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::process::exit;

use clap::{App, Arg, ArgMatches};
use log::{debug, info, warn, error};

use ratchet_version::VERSION;
use exporter::{HIGH_FIVE_COUNTER, NOT_FOUND_COUNTER, register};

const CRLF: &str = "\r\n";

fn main() {
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
                .possible_values(&["trace", "debug", "info", "warn", "error", "crit"])
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

    debug!("booting up log-level: {}", log_level);

    logger::init_logger(String::from(log_level));
    register(Box::new(watcher::get_handler()));

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
    HIGH_FIVE_COUNTER.inc();
    info!("{}", contents);
    (contents, status(200, "OK"))
}

fn handle_404() -> (String, String) {
    let msg = "404 Not Found!";
    NOT_FOUND_COUNTER.inc();
    warn!("{}", msg);
    (msg.to_string(), status(404, "OK"))
}

fn handle_metrics() -> (String, String) {
    // Gather the metrics.
    let output = exporter::gather();

    (output, status(200, "OK"))
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer= [0; 4096];
    stream.read(&mut buffer).unwrap();

    let _matched = |route: &str| {
        debug!("_matched: {} --- {}", String::from_utf8_lossy(&buffer), route);
        matched(&buffer, route)
    };
    let _write = |(contents, status)| write(stream, contents, status);

    if _matched("/") {
        _write(handle_index());
    }else if _matched("/metrics") {
        _write(handle_metrics());
    } else {
        _write(handle_404());
    }
}

fn matched(buffer: &[u8; 4096], route: &str) -> bool {
    // let s = format!("GET {} HTTP/1.1{}", route, CRLF);
    let s = format!("GET {} HTTP/1.", route);
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

