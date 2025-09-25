use crate::http::response::create_response;
use std::collections::HashMap;
use std::net::TcpStream;
use urlencoding::decode;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub _version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,     // ✅ raw bytes, always present
    pub text_body: Option<String>, // ✅ only for text-based requests

    // new fields
    pub extra_response_headers: Vec<String>,
    pub session_id: Option<String>,
}

impl HttpRequest {
    pub fn new() -> Self {
        HttpRequest {
            method: String::new(),
            path: String::new(),
            _version: String::new(),
            headers: HashMap::new(),
            body: None,
            text_body: None,
            extra_response_headers: Vec::new(),
            session_id: None,
        }
    }
    pub fn empty() -> Self {
        HttpRequest {
            method: "".into(),
            path: "".into(),
            _version: "".into(),
            headers: HashMap::new(),
            body: None,
            text_body: None,
            extra_response_headers: Vec::new(),
            session_id: None,
        }
    }
}

pub fn parse_request_from_buffer(
    buffer: &mut Vec<u8>,
    stream: &mut TcpStream,
) -> Option<HttpRequest> {
    if let Some(pos) = twoway::find_bytes(buffer, b"\r\n\r\n") {
        let headers_part = &buffer[..pos];
        let body_start = pos + 4;

        let headers_str = match std::str::from_utf8(headers_part) {
            Ok(s) => s,
            Err(_) => return None,
        };

        let mut lines = headers_str.split("\r\n");
        let request_line = lines.next()?;
        let mut parts = request_line.split_whitespace();
        let method = parts.next()?.to_string();
        let path = parts.next()?.to_string();
        let _version = parts.next()?.to_string();

        let updated_path = match decode(&path) {
            Ok(p) => p.to_string(),
            Err(_) => path,
        };

        let mut headers = HashMap::new();
        for line in lines {
            if let Some((key, value)) = line.split_once(": ") {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        let content_length = headers
            .get("Content-Length")
            .and_then(|cl| cl.parse::<usize>().ok())
            .unwrap_or(0);

        // refuse anything larger than 10MB.
        if content_length > 10 * 1024 * 1024 || buffer.len() > 10 * 1024 * 1024 {
            let resp = create_response(
                "413 Payload Too Large",
                "<h1>413 Payload Too Large</h1>
                <a href=\"/\"><button> Home</button></a>",
                "text/html",
                None,
            );
            resp.send(stream, &HttpRequest::empty());
            return None;
        }

        if buffer.len() < body_start + content_length {
            return None;
        }

        let body = buffer[body_start..body_start + content_length].to_vec();
        let text_body = if let Ok(s) = String::from_utf8(body.clone()) {
            Some(s)
        } else {
            None
        };

        buffer.drain(..body_start + content_length);

        return Some(HttpRequest {
            method,
            path: updated_path,
            _version,
            headers,
            body: Some(body),
            text_body,
            extra_response_headers: Vec::new(),
            session_id: None,
        });
    }

    None
}
