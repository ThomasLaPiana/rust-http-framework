mod configuration;
mod handlers;
mod request;
mod response;

use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::configuration::Config;
use crate::request::Request;

// Handle an incoming connection, including sending a response
fn handle_connection(mut stream: TcpStream) {
    let request = Request::new(&mut stream);
    println!("Parsed Request: {:#?}", request);

    let response = handlers::handle_request(request);
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
