// use std::io::prelude::*;
use std::net::TcpStream;

use crate::errors::handler::error_response;
use crate::http::{request, router};

// handle connection
pub fn handle_connection(mut stream: TcpStream) {
    match request::parse_request(&mut stream) {
        Some(request) => {
            // println!(
            //     "Request:\n method:{} \n path:{} \n version:{} \n headers:{:#?} \n body:{:?} \n",
            //     request.method,
            //     request.path,
            //     request.version,
            //     request.headers,
            //     request.body.unwrap_or_default()
            // );
            println!("method: {} \n for path: {}", request.method, request.path);
            router::route_request(&request.method, &request.path, &mut stream);
        }
        None => {
            eprintln!("Failed to parse request");
            error_response(400, &mut stream);
        }
    }
}
