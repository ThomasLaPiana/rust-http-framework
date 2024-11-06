/// Handle the various request methods that we have implemented
use crate::request::{Request, RequestType};
use crate::response::Response;

// Dispatch the request to the correct handler
pub fn handle_request(request: Request) -> Response {
    match request.info.method {
        RequestType::GET => get(request),
        RequestType::POST => post(request),
        RequestType::DELETE => delete(request),
    }
}

// Handle a GET request
fn get(request: Request) -> Response {
    let mut response = Response::new();
    response.body = format!("GET request for path: '{}'", request.info.path);
    response
}

// Handle a POST request
fn post(request: Request) -> Response {
    let mut response = Response::new();
    response.body = format!("POST request for path: '{}'", request.info.path);
    response
}

// Handle a DELETE request
fn delete(request: Request) -> Response {
    let mut response = Response::new();
    response.body = format!("DELETE request for path: '{}'", request.info.path);
    response
}
