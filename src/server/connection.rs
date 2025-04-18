use std::net::TcpStream;
use std::io::prelude::*;

use crate::http::{request, router};
use crate::errors::handler::error_response;

// handle connection
pub fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer[..]);
            match request::parse_request(&request) {
                Some((method,path, _version, domain)) => {
                    println!("Request: {} {}", method, path);
                    
                    if let Err(e) = router::handle_method(&method, &path, domain.as_deref(), stream) {
                        eprintln!("Error handling request: {}", e);
                    }
                }
                None => {
                    eprintln!("Failed to parse request");
                    error_response(400, stream);
                }
            }
        }
        Err(_) => return,
    }
}
