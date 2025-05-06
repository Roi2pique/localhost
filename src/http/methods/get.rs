use crate::errors::handler::error_response;
use crate::http::response::create_response;
use std::path::Path;
use std::{fs, io::Write, net::TcpStream};

// Handle GET request
pub fn handle_get(path: &str, stream: &mut TcpStream) {
    // For example, you can read a file and send its content as a response
    let sanitize = format!("./ressources/{}", sanitize_path(path));
    let new_path = Path::new(&sanitize);
    if new_path.is_dir() {
        let default = new_path.join("index.html");

        if default.exists() {
            send_file(default, stream);
        } else {
            error_response(404, stream);
        }
    } else if new_path.exists() {
        send_file(new_path.to_path_buf(), stream);
    } else {
        error_response(404, stream);
    }
}

fn sanitize_path(path: &str) -> String {
    path.replace("..", "")
}

fn send_file(path: std::path::PathBuf, stream: &mut TcpStream) {
    match fs::read(&path) {
        Ok(content) => {
            let content_type = match path.extension().and_then(|s| s.to_str()) {
                Some("html") => "text/html",
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                _ => "application/octet-stream",
            };

            let response = create_response("200 OK", content, content_type, None);

            let _ = stream.write_all(response.headers.as_bytes());
            let _ = stream.write_all(&response.body);
        }
        Err(_) => error_response(403, stream),
    }
}
