use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,     // ✅ raw bytes, always present
    pub text_body: Option<String>, // ✅ only for text-based requests
}

impl HttpRequest {
    pub fn body_as_text(&self) -> Option<String> {
        if let Some(text) = &self.text_body {
            Some(text.clone())
        } else if let Some(bytes) = &self.body {
            Some(String::from_utf8_lossy(bytes).to_string())
        } else {
            None
        }
    }
}

pub fn parse_request(stream: &mut TcpStream) -> Option<HttpRequest> {
    let header_stream = stream.try_clone().ok()?;
    let mut reader = BufReader::new(header_stream);

    let mut request_line = String::new();
    reader.read_line(&mut request_line).ok()?;

    let parts: Vec<&str> = request_line.trim_end().split_whitespace().collect();
    if parts.len() != 3 {
        eprintln!("Invalid request line: {}", request_line);
        return None;
    }

    let (method, path, version) = (
        parts[0].to_string(),
        parts[1].to_string(),
        parts[2].to_string(),
    );

    // ✅ Headers
    let mut headers = HashMap::new();
    let mut line = String::new();
    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).ok()?;
        if bytes_read == 0 {
            eprintln!("Unexpected EOF while reading headers");
            return None;
        }

        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            break;
        }

        if let Some((key, value)) = trimmed.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        } else {
            eprintln!("Malformed header line: '{}'", line);
        }
    }

    // ✅ Body
    println!("param : {:#?}\n\n {:#?}", stream, headers);
    let (body, text_body) = read_body(stream, &headers);

    Some(HttpRequest {
        method,
        path,
        version,
        headers,
        body,
        text_body,
    })
}
// there is some of the content of this func that i can reuse to build the parse headers and body ?

fn read_body(
    stream: &mut TcpStream,
    headers: &HashMap<String, String>,
) -> (Option<Vec<u8>>, Option<String>) {
    if let Some(content_length_str) = headers.get("Content-Length") {
        if let Ok(length) = content_length_str.parse::<usize>() {
            let mut buf = vec![0u8; length];
            let mut total_read = 0;
            // debug there
            while total_read < length {
                match stream.read(&mut buf[total_read..]) {
                    Ok(0) => {
                        eprintln!("Connection closed before reading full body");
                        return (None, None);
                    }
                    Ok(n) => {
                        total_read += n;
                        // println!("total read bytes: {}", total_read);
                    }
                    Err(e) => {
                        eprintln!("Failed to read body: {}", e);
                        return (None, None);
                    }
                }
            }

            let body = Some(buf.clone());

            // Decide if we should also decode as text
            let text_body = headers.get("Content-Type").and_then(|ct| {
                if ct.starts_with("text/")
                    || ct.starts_with("application/json")
                    || ct.starts_with("application/xml")
                    || ct.starts_with("application/javascript")
                    || ct.starts_with("application/x-www-form-urlencoded")
                {
                    Some(String::from_utf8_lossy(&buf).to_string())
                } else {
                    None
                }
            });

            return (body, text_body);
        } else {
            eprintln!("Invalid Content-Length: '{}'", content_length_str);
        }
    }

    (None, None)
}

/*use std::io::{BufRead, BufReader, Read};
use std::{collections::HashMap, net::TcpStream};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,             // ✅ raw bytes, always present
    pub text_body: Option<String>, // ✅ only for text-based requests
}

impl HttpRequest {
    // pub fn is_text_based(&self) -> bool {
    //     self.headers.get("Content-Type").map_or(false, |ct| {
    //         ct.starts_with("text/")
    //             || ct.starts_with("application/json")
    //             || ct.starts_with("application/xml")
    //             || ct.starts_with("application/javascript")
    //             || ct.starts_with("application/x-www-form-urlencoded")
    //     })
    // }
}

pub fn parse_request(stream: &mut TcpStream) -> Option<HttpRequest> {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();

    // Read the request line: "POST /path HTTP/1.1"
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

    // Parse headers
    let mut headers = HashMap::new();
    let mut buffer = String::new();
    loop {
        buffer.clear();
        reader.read_line(&mut buffer).ok()?;
        let line = buffer.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(": ") {
            headers.insert(k.to_string(), v.to_string());
        }
    }
    // let mut headers = HashMap::new();
    // let mut line = String::new();
    // while reader.read_line(&mut line).is_ok() {
    //     let trimmed = line.trim_end();
    //     if trimmed.is_empty() {
    //         break;
    //     }
    //     if let Some((key, value)) = trimmed.split_once(": ") {
    //         headers.insert(key.to_string(), value.to_string());
    //     }
    //     line.clear();
    // }

    // Read body if Content-Length is present
    let mut body = Vec::new();
    let mut text_body = None;

    if let Some(cl) = headers.get("Content-Length") {
        if let Ok(length) = cl.parse::<usize>() {
            let mut raw_stream = stream.try_clone().ok()?;
            let mut buf = vec![0u8; length];
            raw_stream.read_exact(&mut buf).ok()?;

            // let mut buf = vec![0; length];
            // if reader.read_exact(&mut buf).is_ok() {
            body = buf;

            // Determine if body is textual based on Content-Type
            if let Some(ct) = headers.get("Content-Type") {
                let is_text = ct.starts_with("text/")
                    || ct.starts_with("application/json")
                    || ct.starts_with("application/xml")
                    || ct.starts_with("application/javascript")
                    || ct.starts_with("application/x-www-form-urlencoded");

                if is_text {
                    text_body = Some(String::from_utf8_lossy(&body).to_string());
                }
            }
        }
    }

    Some(HttpRequest {
        method,
        path,
        version,
        headers,
        body,
        text_body,
    })
}
    */
// old one
//
// use std::io::Read;
// use std::io::{BufRead, BufReader};
// use std::{collections::HashMap, net::TcpStream};
// #[derive(Debug)]
// pub struct HttpRequest {
//     pub method: String,
//     pub path: String,
//     pub version: String,
//     pub headers: HashMap<String, String>,
//     pub body: Option<String>,
// }
// pub fn parse_request(stream: &mut TcpStream) -> Option<HttpRequest> {
//     let mut reader = BufReader::new(stream);
//     let mut request_line = String::new();
//     if reader.read_line(&mut request_line).is_err() {
//         return None;
//     }
//     let parts: Vec<&str> = request_line.trim_end().split_whitespace().collect();
//     if parts.len() != 3 {
//         return None;
//     }
//     let (method, path, version) = (
//         parts[0].to_string(),
//         parts[1].to_string(),
//         parts[2].to_string(),
//     );
//     let mut headers = HashMap::new();
//     let mut line = String::new();
//     while reader.read_line(&mut line).is_ok() {
//         let trimmed = line.trim_end(); // Don't overwrite the original String
//         if trimmed.is_empty() {
//             break;
//         }
//         if let Some((key, value)) = trimmed.split_once(": ") {
//             headers.insert(key.to_string(), value.to_string());
//         }
//         line.clear(); // Works because `line` is still the mutable String
//     }
//     let mut body = None;
//     if let Some(content_length) = headers.get("Content-Length") {
//         if let Ok(len) = content_length.parse::<usize>() {
//             let mut buf = vec![0; len];
//             if reader.read_exact(&mut buf).is_ok() {
//                 body = Some(String::from_utf8_lossy(&buf).to_string());
//             }
//         }
//     }
//     Some(HttpRequest {
//         method,
//         path,
//         version,
//         headers,
//         body,
//     })
// }
