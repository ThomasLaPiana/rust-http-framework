/// Handle the Request from the client
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

#[derive(Debug)]
pub enum RequestType {
    GET,
    POST,
    DELETE,
}

impl RequestType {
    // Convert a string to a RequestType
    fn from_string(method: &str) -> RequestType {
        match method {
            "GET" => RequestType::GET,
            "POST" => RequestType::POST,
            "DELETE" => RequestType::DELETE,
            _ => panic!("Invalid Request Type"),
        }
    }
}

// A representation of the first line of an HTTP Request
#[derive(Debug)]
pub struct RequestInfo {
    pub method: RequestType,
    pub path: String,
    pub protocol: String,
}

impl RequestInfo {
    // Parse the first line of the HTTP Request into its discrete components
    pub fn new(line: &str) -> RequestInfo {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [method, path, protocol] => RequestInfo {
                method: RequestType::from_string(method),
                path: path.to_string(),
                protocol: protocol.to_string(),
            },
            _ => panic!("Invalid Request"),
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub info: RequestInfo,
    pub headers: Vec<String>,
    pub body: String,
}

impl Request {
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

    // Parse the metadata and headers for the request
    pub fn parse_info_and_headers(reader: &mut BufReader<TcpStream>) -> (RequestInfo, Vec<String>) {
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
            .take_while(|l| !l.is_empty())
            .collect();

        let info = RequestInfo::new(&lines[0].clone());
        let headers = lines[1..].to_vec();
        return (info, headers);
    }

    // Read a full request from a stream
    pub fn new(stream: &mut TcpStream) -> Request {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let (info, headers) = Request::parse_info_and_headers(&mut reader);
        let content_length = Request::get_content_length(&headers);

        let mut body_buffer = vec![0; content_length as usize];
        reader.read_exact(&mut body_buffer).unwrap();
        let body_lines: Vec<String> = String::from_utf8(body_buffer)
            .unwrap()
            .split("\n")
            .map(|l| l.to_owned())
            .collect();

        Request {
            info: info,
            headers: headers,
            body: body_lines.join("\n"),
        }
    }
}
