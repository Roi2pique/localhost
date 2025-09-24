use crate::errors::handler::error_response;
use crate::http::request::HttpRequest;
use crate::http::response::create_response;
use crate::http::utils;
use std::path::Path;
use std::{fs, net::TcpStream};

// Handle GET request
pub fn handle_get(req: &HttpRequest, stream: &mut TcpStream) {
    let path = req.path.as_str();
    let base_dir = Path::new("./ressources");
    let src = Path::new("./src");

    let fs_path = match utils::sanitize_path(path, base_dir) {
        Some(p) => p,
        None => {
            error_response(403, stream);
            return;
        }
    };

    let index_path = base_dir.join("index.html");
    let upload_dir = base_dir.join("upload");
    let script_dir = src.join("cgi_bin/scripts");

    if path == "/" {
        match utils::render_home_with_two_lists(
            &index_path,
            &upload_dir,
            &script_dir,
            "/upload",
            "/scripts",
        ) {
            Some(content) => {
                let response = create_response("200 OK", content.into_bytes(), "text/html", None);
                response.send(stream, &req);
            }
            None => {
                error_response(500, stream); // couldn't read or build template
            }
        }
    }

    if fs_path.is_dir() {
        let default = fs_path.join("index.html");

        if default
            .try_exists()
            .expect("Can't check existence of file 1")
        {
            send_file(default, stream, req);
        } else {
            error_response(404, stream);
        }
    } else if fs_path
        .try_exists()
        .expect("Can't check existence of file 2")
    {
        send_file(fs_path.to_path_buf(), stream, req);
    } else {
        error_response(404, stream);
    }
}

fn send_file(path: std::path::PathBuf, stream: &mut TcpStream, req: &HttpRequest) {
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
            response.send(stream, &req);
        }
        Err(_) => {
            println!("error : sendfile");
            error_response(403, stream);
        }
    }
}
