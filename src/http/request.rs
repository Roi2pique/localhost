use std::collections::HashMap;

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

// impl HttpRequest {
//     pub fn body_as_text(&self) -> Option<String> {
//         if let Some(text) = &self.text_body {
//             Some(text.clone())
//         } else if let Some(bytes) = &self.body {
//             Some(String::from_utf8_lossy(bytes).to_string())
//         } else {
//             None
//         }
//     }
// }

pub fn parse_request_from_buffer(buffer: &mut Vec<u8>) -> Option<HttpRequest> {
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
            path,
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
