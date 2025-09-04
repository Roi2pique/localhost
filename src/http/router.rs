use crate::cgi_bin::handle;
use crate::errors::handler::error_response;
use crate::http::methods::*;
use std::net::TcpStream;

use super::request::HttpRequest;

pub const RESOURCES_DIR: &str = "./ressources";
pub const UPLOAD_DIR: &str = "upload";

pub fn route_request(req: HttpRequest, stream: &mut TcpStream) {
    if req.path.starts_with("/scripts/") {
        handle::handle_cgi(&req, stream);
        return;
    }

    match req.method.as_str() {
        "GET" => get::handle_get(&req, stream),
        "POST" => post::handle_post(&req, stream),
        "DELETE" => delete::handle_delete(&req, stream),
        _ => error_response(405, stream),
    }
}
