use crate::errors::handler::error_response;
use crate::http::methods::*;
use std::net::TcpStream;

pub fn route_request(method: &str, path: &str, stream: &mut TcpStream) {
    println!("method: {}", method);
    match method {
        "GET" => get::handle_get(path, stream),
        "POST" => post::handle_post(path, stream),
        "DELETE" => delete::handle_delete(path, stream),
        _ => {
            error_response(405, stream);
        }
    }
}
