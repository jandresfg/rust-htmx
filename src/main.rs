use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = match buffer {
        s if s.starts_with(b"GET /frameworks") => ("200 OK", "frameworks.html"),
        s if s.starts_with(b"GET /") => ("200 OK", "index.html"),
        _ => ("404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "HTTP/1.1 {}\r\nContent-length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
