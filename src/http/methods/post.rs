use log::error;
use regex::Regex;
use std::fs::File;
use std::{io::Write, net::TcpStream};

use crate::http::response::create_response;
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
    // println!("HTTPRequest {:?}\n", req.body);
    let body_bytes = match &req.body {
        Some(body) => body,
        None => {
            error_response(400, stream);
            error!("Missing body in request {:#?}", req);
            return;
        }
    };

    match parse_multipart_form(body_bytes, &boundary) {
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

            let response = create_response("200 OK", html, "text/html", None);

            response.send(stream, req);
            println!("the filename : {:?}", filename);
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

pub fn parse_multipart_form(body: &[u8], boundary: &str) -> Option<(String, Vec<u8>)> {
    let boundary_marker = format!("--{}", boundary);
    let boundary_bytes = boundary_marker.as_bytes();

    let mut start = 0;
    while let Some(pos) = twoway::find_bytes(&body[start..], boundary_bytes) {
        let part_start = start + pos + boundary_bytes.len();
        // skip leading \r\n
        let part = &body[part_start..];

        if let Some(headers_end) = twoway::find_bytes(part, b"\r\n\r\n") {
            let (headers_raw, content_raw) = part.split_at(headers_end);
            let headers_str = String::from_utf8_lossy(headers_raw);

            if headers_str.contains("filename=") {
                let filename_re = Regex::new(r#"filename="([^"]+)""#).ok()?;
                let filename = filename_re
                    .captures(&headers_str)?
                    .get(1)?
                    .as_str()
                    .to_string();

                // skip \r\n\r\n
                let mut content = content_raw[4..].to_vec();

                // trim trailing CRLF
                while content.ends_with(&[b'\r', b'\n']) {
                    content.truncate(content.len().saturating_sub(2));
                }

                return Some((filename, content));
            }
        }
        start = part_start;
    }

    None
}

fn extract_boundary(header: &str) -> Option<String> {
    header
        .split("boundary=")
        .nth(1)
        .map(|b| b.trim().to_string())
}
