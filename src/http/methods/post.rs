use log::error;
use regex::Regex;
use std::fs::{create_dir_all, File};
use std::{io::Write, net::TcpStream};

use crate::http::router::{RESOURCES_DIR, UPLOAD_DIR};
use crate::{errors::handler::error_response, http::request::HttpRequest};

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

    let body = match &req.body {
        Some(b) => b,
        None => {
            error_response(400, stream);
            error!("Missing body in request {:#?}", req);
            return;
        }
    };
    println!(
        "body: {} and boundary: {} and the parse: {:?}",
        body,
        boundary,
        parse_multipart_form(body, &boundary)
    );
    match parse_multipart_form(body, &boundary) {
        Some((filename, file_bytes)) => {
            // create_dir(UPLOAD_DIR, Some(RESOURCES_DIR)); // ensure folder exists

            let save_path = format!("./{}/{}", UPLOAD_DIR, filename);
            if let Ok(mut file) = File::create(&save_path) {
                let _ = file.write_all(&file_bytes);
            } else {
                error_response(500, stream);
                error!("Failed to save file at {}", save_path);
                return;
            }

            let html = format!("<h1>Upload complete</h1><p>Saved as: {}</p>", filename);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                html.len(),
                html
            );
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

/// Parses a multipart/form-data body with a single file input field
pub fn parse_multipart_form(body: &str, boundary: &str) -> Option<(String, Vec<u8>)> {
    // let boundary_regex = format!(r"--{}(?:--)?", regex::escape(boundary));

    // Regex to match file upload part
    let re = Regex::new(&format!(
        r#"--{}\r\nContent-Disposition: form-data; name="file"; filename="([^"]+)"\r\nContent-Type: [^\r\n]+\r\n\r\n(.*?)\r\n--{}"#,
        regex::escape(boundary),
        regex::escape(boundary)
    )).ok()?;

    let caps = re.captures(body)?;
    println!("caps: {:?}", caps);
    let filename = caps.get(1)?.as_str().to_string();
    let content = caps.get(2)?.as_str();

    Some((filename, content.as_bytes().to_vec()))
}

fn extract_boundary(header: &str) -> Option<String> {
    header
        .split("boundary=")
        .nth(1)
        .map(|b| b.trim().to_string())
}

// pub fn create_dir(path: &str, parent: Option<&str>) {
//     let full_path = if let Some(parent_dir) = parent {
//         format!("{}/{}", parent_dir, path)
//     } else {
//         path.to_string()
//     };

//     let dir_path = Path::new(&full_path);

//     if !dir_path.exists() {
//         if let Err(e) = create_dir_all(&full_path) {
//             error!("Failed to create directory {}: {}", full_path, e);
//         } else {
//             println!("Created directory: {}", full_path);
//         }
//     } else {
//         println!("Directory {} already exists", full_path);
//     }
// }
