use std::net::TcpStream;

use crate::errors::handler::error_response;
use crate::http::{request, router};

// handle connection
pub fn handle_connection(mut stream: TcpStream) {
    match request::parse_request(&mut stream) {
        Some(request) => {
            // info!("request: {:#?}", request);

            println!("method: {} for path: {}", request.method, request.path);
            router::route_request(request, &mut stream);
        }
        None => {
            eprintln!("Failed to parse request");
            error_response(400, &mut stream);
        }
    }
}
