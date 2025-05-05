use std::collections::HashMap;

pub struct HttpResponse {
    pub headers: String,
    pub body: Vec<u8>,
}

// Builds proper HTTP responses
pub fn create_response(
    status: &str,
    body: impl Into<Vec<u8>>,
    content_type: &str,
    additional_headers: Option<HashMap<&str, &str>>,
) -> HttpResponse {
    let body = body.into();
    let mut headers = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n",
        status,
        body.len(),
        content_type
    );

    if let Some(extra) = additional_headers {
        for (key, value) in extra {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }
    }

    headers.push_str("\r\n");

    HttpResponse { headers, body }
}

// pub fn create_response(status: &str, body: &str) -> String {
//     format!(
//         "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
//         status,
//         body.len(),
//         body
//     )
// }

// use std::net::TcpStream;
// use std::fs;
// pub fn serve_file(path: &str, _: &mut TcpStream) -> String {
//     match fs::read_to_string(path) {
//         Ok(content) => create_response("200 OK", &content),
//         Err(e) => create_response("404 Not Found", e.to_string().as_str()),
//     }
// }
