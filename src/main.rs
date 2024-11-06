mod configuration;
mod database;
mod handlers;
mod request;
mod response;

use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use uuid::Uuid;

use crate::configuration::Config;
use crate::database::Database;
use crate::request::Request;

// Handle an incoming connection, including sending a response
fn handle_connection(mut stream: TcpStream, database: &Database) {
    let rid = Uuid::new_v4().simple().to_string()[..8].to_string();
    println!("({}) | Received Connection", rid);

    let request = Request::new(&mut stream);
    println!(
        "({}) | Processing Request: {:?} - {}",
        rid, request.info.method, request.info.path
    );

    let response = handlers::handle_request(request, database);
    println!(
        "({}) | Processed Request with Status: {}",
        rid, response.status_code
    );
    stream.write_all(&response.as_bytes()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args);
    println!("(Host: '{}', Port: '{}')", config.host, config.port);

    // Create a new database
    let database = Database::new();

    println!("> Starting server...");
    let listener = TcpListener::bind(config.addr()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &database);
    }
}
