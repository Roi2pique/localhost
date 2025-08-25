use log::error;
use regex::Regex;
use std::fs::File;
use std::{io::Write, net::TcpStream};

use crate::{
    errors::handler::error_response,
    http::request::HttpRequest,
    http::router::{RESOURCES_DIR, UPLOAD_DIR},
};

// Handle POST request
pub fn handle_post(req: &HttpRequest, stream: &mut TcpStream) {
    match req.path.as_str() {
        "/upload" => handle_upload(req, stream),
        _ => {
            error_response(404, stream);
            error!("POST request to unknown path: {}", req.path);
        }
    }
}

pub fn handle_upload(req: &HttpRequest, stream: &mut TcpStream) {
    let binding = "".into();
    let content_type = req.headers.get("Content-Type").unwrap_or(&binding);

    if !content_type.starts_with("multipart/form-data") {
        error_response(400, stream);
        error!("Expected multipart/form-data, got: {:?}", content_type);
        return;
    }

    let boundary = extract_boundary(content_type);
    if boundary.is_none() {
        error_response(400, stream);
        error!("Missing boundary in Content-Type");
        return;
    }
    let boundary = boundary.unwrap();
    println!("HTTPRequest {:#?}\n", req.body);
    let body_bytes = match req.body_as_text() {
        // modif for the bytes
        Some(body) => body,
        None => {
            error_response(400, stream);
            error!("Missing body in request {:#?}", req);
            return;
        }
    };

    match parse_multipart_form(&body_bytes, &boundary) {
        Some((filename, file_bytes)) => {
            let is_script = filename.ends_with(".py")
                || filename.ends_with(".sh")
                || filename.ends_with(".cgi");

            // Decide where to save
            let save_dir = if is_script {
                "src/cgi_bin/scripts"
            } else {
                &format!("{}/{}", RESOURCES_DIR, UPLOAD_DIR)
            };

            let save_path = format!("{}/{}", save_dir, filename);

            match File::create(&save_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(&file_bytes) {
                        error_response(500, stream);
                        error!("Failed to write file: {}", e);
                        return;
                    }
                }
                Err(e) => {
                    error_response(500, stream);
                    error!("Failed to create file at {}: {}", save_path, e);
                    return;
                }
            }

            let html = format!(
                "<h1>Upload complete</h1><p>Saved as: {}</p>
                <a href= \"/\"><button> Home</button></a>",
                filename
            );
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                html.len(),
                html
            );
            println!("the filename : {:?}", filename);
            let _ = stream.write_all(response.as_bytes());
        }
        None => {
            error_response(400, stream);
            error!(
                "Failed to parse multipart form data with request: {:#?}",
                req
            );
        }
    }
}

// next there is the new version
pub fn parse_multipart_form(body: &str, boundary: &str) -> Option<(String, Vec<u8>)> {
    let boundary_marker = format!("--{}", boundary);

    // Split body into parts
    let parts: Vec<&str> = body.split(&boundary_marker).collect();

    for part in parts {
        if part.contains("Content-Disposition") && part.contains("filename=") {
            let filename_re = Regex::new(r#"filename="([^"]+)""#).ok()?;
            let filename = filename_re.captures(part)?.get(1)?.as_str().to_string();

            let split = part.split("\r\n\r\n").collect::<Vec<_>>();
            if split.len() < 2 {
                continue; // malformed part
            }

            let content_part = split[1];
            // Remove potential trailing line breaks / boundary indicators
            let content = content_part.trim_end_matches("\r\n").trim_end();

            return Some((filename, content.as_bytes().to_vec()));
        }
    }

    None
}

fn extract_boundary(header: &str) -> Option<String> {
    header
        .split("boundary=")
        .nth(1)
        .map(|b| b.trim().to_string())
}
