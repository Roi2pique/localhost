use std::{
    net::TcpStream,
    io::Write,
};
use crate::http::response::create_response;

// Generates 404, 403, etc.
pub fn error_response(code: u16, client: &mut TcpStream) {
    let resp = match code {
        400 => create_response("400 Bad Request", "<h1>Error 400 - Bad Request</h1>"),
        403 => create_response("403 Forbidden", "<h1>Error 403 - Forbidden</h1>"),
        404 => create_response("404 Not Found", "<h1>Error 404 - Not Found</h1>"),
        405 => create_response("405 Method Not Allowed", "<h1>Error 405 - Method Not Allowed</h1>"),
        413 => create_response("413 Payload Too Large", "<h1>Error 413 - Payload Too Large</h1>"),
        500 => create_response("500 Internal Server Error", "<h1>500 - Internal Server Error</h1>"),
        _ => create_response("000 Unknwon Error", "<h1>000 - Unknwon Error</h1>"),
    };
    if let Err(e) = client.write(resp.as_bytes()) {
        eprintln!("500 Internal error server: {}", e);
    }
}
