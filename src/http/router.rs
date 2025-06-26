use crate::errors::handler::error_response;
use crate::http::methods::*;
use std::net::TcpStream;

use super::request::HttpRequest;

pub const RESOURCES_DIR: &str = "./ressources";
pub const UPLOAD_DIR: &str = "upload";

pub fn route_request(request: HttpRequest, stream: &mut TcpStream) {
    match request.method.as_str() {
        "GET" => get::handle_get(&request.path, stream),
        "POST" => post::handle_post(&request, stream),
        "DELETE" => delete::handle_delete(&request, stream),
        _ => {
            error_response(405, stream);
        }
    }
}
