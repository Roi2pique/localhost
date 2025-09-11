use crate::cgi_bin::handle;
use crate::errors::handler::error_response;
use crate::http::methods::*;
use crate::http::response::create_response;
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
        "GET" if req.path == "/empty" => {
            let response = create_response("200 OK", "", "text/plain", None);
            response.send(stream, &req);
        }
        "GET" => get::handle_get(&req, stream),
        "POST" => post::handle_post(&req, stream),
        "DELETE" => delete::handle_delete(&req, stream),
        _ => error_response(405, stream),
    }
}
