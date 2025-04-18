
// Builds proper HTTP responses
pub fn create_response(status: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        body.len(),
        body
    )
}

// use std::net::TcpStream;
// use std::fs;

// pub fn serve_file(path: &str, _: &mut TcpStream) -> String {
//     match fs::read_to_string(path) {
//         Ok(content) => create_response("200 OK", &content),
//         Err(e) => create_response("404 Not Found", e.to_string().as_str()),
//     }
// }