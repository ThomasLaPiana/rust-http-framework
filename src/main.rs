mod configuration;
mod database;
mod handlers;
mod request;
mod response;

use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::configuration::Config;
use crate::database::Database;
use crate::request::Request;

// Handle an incoming connection, including sending a response
fn handle_connection(mut stream: TcpStream, database: &Database) {
    let request = Request::new(&mut stream);
    let response = handlers::handle_request(request, database);
    stream.write_all(&response.as_bytes()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args);
    println!("(Host: '{}', Port: '{}')", config.host, config.port);

    // Create a new database
    let database = Database::new();

    println!("Starting server...");
    let listener = TcpListener::bind(config.addr()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &database);
    }
}
