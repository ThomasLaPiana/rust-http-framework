/// Handle the Response to the client

#[derive(Debug)]
pub struct Response {
    pub version: String,
    pub status_code: u32,
    status_text: String,
    pub body: String,
}

impl Response {
    pub fn new() -> Response {
        Response {
            version: "HTTP/1.1".to_string(),
            status_code: 200,
            status_text: "OK".to_string(),
            body: "".to_string(),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        format!(
            "{} {} {}\r\ncontent-type: text/plain\r\n\r\n{}",
            self.version, self.status_code, self.status_text, self.body
        )
        .into_bytes()
    }
}
