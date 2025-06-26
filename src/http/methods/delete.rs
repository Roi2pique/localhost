use crate::errors::handler::error_response;
use crate::http::request::HttpRequest;
use crate::http::utils::sanitize_path;
use log::error;
use std::net::TcpStream;
use std::path::Path;
use std::{fs, io::Write};

pub fn handle_delete(req: &HttpRequest, stream: &mut TcpStream) {
    let base_dir = Path::new("./ressources");
    let sanitized = match sanitize_path(&req.path, base_dir) {
        Some(p) => p,
        None => {
            error_response(403, stream);
            return;
        }
    };
    if !Path::new(&sanitized).exists() {
        error_response(404, stream);
        return;
    }

    println!("Deleting file: {}", sanitized.display());
    match fs::remove_file(sanitized) {
        Ok(_) => {
            let msg = format!("<h1>File deleted successfully</h1><p>{}</p>", req.path);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                msg.len(),
                msg
            );
            let _ = stream.write_all(response.as_bytes());
        }
        Err(e) => {
            error_response(500, stream);
            error!("Failed to delete file: {}. Error: {}", req.path, e);
            return;
        }
    }
}
