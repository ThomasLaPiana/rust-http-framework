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

// Get the Content Length from the request
fn get_content_length(request: &Vec<String>) -> i16 {
    let content_length_str = "Content-Length: ";
    for line in request {
        if line.starts_with(content_length_str) {
            let content_length: &str = line.split(": ").collect::<Vec<_>>()[1];
            return content_length.parse::<i16>().unwrap();
        }
    }

    return 0;
}
// Read a full request from a stream
fn read_stream(stream: &mut TcpStream) -> Vec<String> {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut buffer = String::new();

    loop {
        let r = reader.read_line(&mut buffer).unwrap();
        // Detect an empty line (\0 null byte)
        if r < 3 {
            break;
        }
    }
    let lines: Vec<String> = buffer
        .split("\n")
        .map(|l| l.trim_end_matches("\r").to_string())
        .collect();
    println!("Lines: {:#?}", lines);

    // Read the exact Content Length
    let content_length = get_content_length(&lines);

    let mut body_buffer = vec![0; content_length as usize];
    reader.read_exact(&mut body_buffer).unwrap();
    let body_lines: Vec<String> = String::from_utf8(body_buffer)
        .unwrap()
        .split("\n")
        .map(|l| l.to_owned())
        .collect();

    [lines.clone(), body_lines].concat()
}

// Handle TCP Connections
fn handle_connection(mut stream: TcpStream) {
    let raw_request = read_stream(&mut stream);
    println!("Raw Request: {:#?}", raw_request);
    // println!("Raw Request: {http_request:#?}");
    // let request = Request::new(http_request.clone());
    // println!("Parsed Request: {:#?}", request);

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
