/// Handle the Request from the client

// A representation of the first line of an HTTP Request
#[derive(Debug)]
pub struct RequestInfo {
    pub method: String,
    pub path: String,
    pub protocol: String,
}

impl RequestInfo {
    // Parse the first line of the HTTP Request into its discrete components
    pub fn new(line: &str) -> RequestInfo {
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
pub struct Request {
    pub info: RequestInfo,
    pub headers: Vec<String>,
}

impl Request {
    pub fn new(request: Vec<String>) -> Request {
        let info = RequestInfo::new(&request[0].clone());
        Request {
            info: info,
            headers: request[1..].to_vec(),
        }
    }
}
