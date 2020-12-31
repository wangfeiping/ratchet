use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

const CRLF: &str = "\r\n";
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| handle_connection(stream));
    }
}

fn handle_index() -> (String, String) {
    let contents = "Hello!";
    (contents.to_string(), status(200, "OK"))
}

fn handle_404() -> (String, String) {
    let warn = "404 Not Found!";
    (warn.to_string(), status(404, "OK"))
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

