use crate::database::Database;
/// Handle the various request methods that we have implemented
use crate::request::{Request, RequestType};
use crate::response::Response;

// Dispatch the request to the correct handler
pub fn handle_request(request: Request, database: &Database) -> Response {
    match request.info.method {
        RequestType::GET => {
            let mut response = Response::new();
            match database.get(&request.info.path) {
                Some(value) => {
                    response.body = value;
                    response
                }
                None => {
                    response.body = "No value found".to_string();
                    response.status_code = 404;
                    response
                }
            }
        }
        RequestType::POST => {
            let mut response = Response::new();
            database.set(request.info.path.clone(), request.body.clone());
            response.status_code = 201;
            response.body = request.body.clone();
            response
        }
        RequestType::DELETE => {
            let mut response = Response::new();
            database.delete(&request.info.path);
            response.status_code = 204;
            response
        }
    }
}
