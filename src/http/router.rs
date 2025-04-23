use crate::errors::handler::error_response;
use crate::http::methods::*;
use std::net::TcpStream;

pub fn handle_method(
    method: &str,
    path: &str,
    _domain: Option<&str>,
    stream: &mut TcpStream,
) -> Result<(), String> {
    match method {
        "GET" => {
            get::handle_get(path, stream); // Add domain if needed
            Ok(())
        }
        "POST" => {
            post::handle_post(path, stream);
            Ok(())
        }
        "DELETE" => {
            delete::handle_delete(path, stream);
            Ok(())
        }
        _ => {
            error_response(405, stream);
            Ok(())
        }
    }
}
