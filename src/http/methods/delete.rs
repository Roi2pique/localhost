use crate::errors::handler::error_response;
use crate::http::request::HttpRequest;
use crate::http::response::create_response;
use crate::http::utils::sanitize_path;
use log::error;
use std::fs;
use std::net::TcpStream;
use std::path::Path;

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
            let response = create_response("200 OK", msg, "text/html", None);

            response.send(stream, req);
        }
        Err(e) => {
            error_response(500, stream);
            error!("Failed to delete file: {}. Error: {}", req.path, e);
            return;
        }
    }
}
/* modify for path if is a script
let is_script = filename.ends_with(".py")
                || filename.ends_with(".sh")
                || filename.ends_with(".cgi");

     */
