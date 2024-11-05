use std::env;
use std::io::{prelude::*, BufReader};
use std::net::TcpListener;
use std::net::TcpStream;

#[derive(Debug)]
struct Config {
    host: String,
    port: String,
}

impl Config {
    fn addr(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

// A representation of the first line of an HTTP Request
#[derive(Debug)]
struct RequestInfo {
    method: String,
    path: String,
    protocol: String,
}

impl RequestInfo {
    // Parse the first line of the HTTP Request into its discrete components
    fn new(line: &str) -> RequestInfo {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [method, path, protocol] => RequestInfo {
                method: method.to_string(),
                path: path.to_string(),
                protocol: protocol.to_string(),
            },
            _ => panic!("Invalid Request"),
        }
    }
}

#[derive(Debug)]
struct Request {
    info: RequestInfo,
    headers: Vec<String>,
}

impl Request {
    fn new(request: Vec<String>) -> Request {
        let info = RequestInfo::new(&request[0].clone());
        Request {
            info: info,
            headers: request[1..].to_vec(),
        }
    }
}

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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = get_config(args);
    println!("(Host: '{}', Port: '{}')", config.host, config.port);

    println!("Starting server...");
    let listener = TcpListener::bind(config.addr()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

// Get the configuration from the command line arguments
fn get_config(args: Vec<String>) -> Config {
    // Set the default host & port
    let host = "127.0.0.1".to_string();
    let port = "8080".to_string();

    match args.len() {
        1 => {
            println!("> No arguments provided, using default values");
            return Config {
                host: host,
                port: port,
            };
        }
        2 => {
            println!("> No Port provided, using default value");
            return Config {
                host: args[2].clone(),
                port: port,
            };
        }
        3 => {
            println!("> Host and Port args provided");
            return Config {
                host: args[2].clone(),
                port: args[3].clone(),
            };
        }
        _ => {
            panic!("Too many arguments provided");
        }
    }
}
