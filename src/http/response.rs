use crate::errors::handler::error_response;
use crate::http::request::HttpRequest;
use log::error;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

#[derive(Debug)]
pub struct HttpResponse {
    pub headers: String,
    pub body: Vec<u8>,
    pub extra_headers: Vec<String>,
}

impl HttpResponse {
    pub fn send(&self, stream: &mut TcpStream, req: &HttpRequest) {
        let mut headers = self.headers.clone();

        // Inject extra headers from middleware (sessions etc.)
        for h in &req.extra_response_headers {
            headers.push_str(h);
            headers.push_str("\r\n");
        }

        // Also include response-specific extra headers (if any)
        for h in &self.extra_headers {
            headers.push_str(h);
            headers.push_str("\r\n");
        }

        headers.push_str("\r\n");

        if let Err(e) = stream.write_all(headers.as_bytes()) {
            error!("write failed (headers) : {}", e);
            error_response(413, stream);
            return;
        };
        if let Err(e) = stream.write_all(&self.body) {
            error!("write failed (body) : {}", e)
        };
    }
}

// Builds proper HTTP responses
pub fn create_response(
    status: &str,
    body: impl Into<Vec<u8>>,
    content_type: &str,
    additional_headers: Option<HashMap<&str, &str>>,
) -> HttpResponse {
    let body = body.into();
    let mut headers = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n",
        status,
        body.len(),
        content_type
    );

    if let Some(extra) = additional_headers {
        for (key, value) in extra {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }
    }

    HttpResponse {
        headers,
        body,
        extra_headers: Vec::new(),
    }
}
