mod configuration;
mod request;
mod response;

use std::env;
use std::io::{prelude::*, BufReader};
use std::net::TcpListener;
use std::net::TcpStream;

use crate::configuration::Config;
use crate::request::Request;
use crate::response::Response;

// Handle TCP Connections
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Raw Request: {http_request:#?}");
    let request = Request::new(http_request.clone());
    println!("Parsed Request: {:#?}", request);

    let response = Response::new();
    stream.write_all(&response.as_bytes()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args);
    println!("(Host: '{}', Port: '{}')", config.host, config.port);

    println!("Starting server...");
    let listener = TcpListener::bind(config.addr()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
