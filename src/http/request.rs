use std::io::Read;
use std::io::{BufRead, BufReader};
use std::{collections::HashMap, net::TcpStream};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

pub fn parse_request(stream: &mut TcpStream) -> Option<HttpRequest> {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();

    if reader.read_line(&mut request_line).is_err() {
        return None;
    }

    let parts: Vec<&str> = request_line.trim_end().split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }

    let (method, path, version) = (
        parts[0].to_string(),
        parts[1].to_string(),
        parts[2].to_string(),
    );

    let mut headers = HashMap::new();
    let mut line = String::new();

    while reader.read_line(&mut line).is_ok() {
        let trimmed = line.trim_end(); // Don't overwrite the original String
        if trimmed.is_empty() {
            break;
        }
        if let Some((key, value)) = trimmed.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }

        line.clear(); // Works because `line` is still the mutable String
    }

    let mut body = None;
    if let Some(content_length) = headers.get("Content-Length") {
        if let Ok(len) = content_length.parse::<usize>() {
            let mut buf = vec![0; len];
            if reader.read_exact(&mut buf).is_ok() {
                body = Some(String::from_utf8_lossy(&buf).to_string());
            }
        }
    }

    Some(HttpRequest {
        method,
        path,
        version,
        headers,
        body,
    })
}
